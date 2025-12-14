
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::{task::Poll, time::Duration};

use futures::future::{select, Either, Map, Select};
use futures::task::waker;
use futures::{ready, FutureExt, Sink, SinkExt, Stream, StreamExt, TryStream};
use oozebot_protocol::close_codes::GatewayCloseCode;
use oozebot_protocol::events::receive::{self, GatewayCloseEvent, GatewayIncoming, GatewayRecvEvent};
use oozebot_protocol::events::send::{self, GatewaySendEvent};
use oozebot_protocol::{GatewayError, HeartbeatError, RawGatewayPayload};
use pin_project_lite::pin_project;
use tokio::sync::{OwnedRwLockReadGuard, RwLock};
use tokio::time::{interval, Interval};
use tokio_tungstenite::tungstenite::{self, Message};

use crate::streams::{Duplex, FanInSink, ReconnectManager, StreamExtSplit};



impl From<receive::Heartbeat> for Heartbeat {
    fn from(value: receive::Heartbeat) -> Self {
        Heartbeat { latest_sequence_number: value.seq }
    }
}

impl From<receive::Heartbeat> for HeartbeatManagerInput {
    fn from(value: receive::Heartbeat) -> Self {
        HeartbeatManagerInput::Heartbeat(value.into())
    }
}

impl From<receive::HeartbeatAck> for HeartbeatAck {
    fn from(_value: receive::HeartbeatAck) -> Self {
        HeartbeatAck {  }
    }
}

impl From<receive::HeartbeatAck> for HeartbeatManagerInput {
    fn from(value: receive::HeartbeatAck) -> Self {
        HeartbeatManagerInput::HeartbeatAck(value.into())
    }
}

impl From<Heartbeat> for GatewaySendEvent {
    fn from(value: Heartbeat) -> Self {
        let heartbeat = send::Heartbeat{
            d: value.latest_sequence_number
        };
        GatewaySendEvent::Heartbeat(heartbeat)
    }
}

type Seq = Option<u64>;

pub struct GatewaySession {
    sequence_number: Seq,
}

impl GatewaySession {
    fn new() -> GatewaySession {
        GatewaySession {
            sequence_number: None,
        }
    }
}

async fn decode_message(msg: Result<Message, tungstenite::Error>, gateway_session: Arc<RwLock<GatewaySession>>) -> Result<GatewayIncoming, GatewayError> {

    match msg {
        Ok(Message::Text(text)) => {
            match serde_json::from_str::<RawGatewayPayload>(&text) {
                Ok(raw) => {
                    if raw.s.is_some() {
                        let mut session_write = gateway_session.write().await;
                        session_write.sequence_number = raw.s;
                    }
                    match TryInto::<GatewayRecvEvent>::try_into(raw) {
                        Ok(event) => return Ok(GatewayIncoming::Recv(event)),
                        Err(e) => return Err(Into::<GatewayError>::into(e))
                    }
                },
                Err(e) => return Err(Into::<GatewayError>::into(e))
            }
        },
        Ok(Message::Close(maybe_frame)) => {
            let close_event = Into::<GatewayCloseEvent>::into(maybe_frame);
            return Ok(GatewayIncoming::Close(close_event))
        },
        Ok(msg) => {panic!("Received an unexpected message type from Discord gateway: {:?}", msg)},
        Err(e) => return Err(Into::<GatewayError>::into(e))
    };
}

pub async fn create_connection(url: &str, gateway_session: Arc<RwLock<GatewaySession>>) -> (impl Sink<GatewaySendEvent, Error = GatewayError>, impl Stream<Item = Result<GatewayIncoming, GatewayError>>) {
    let (ws_connection, _response) = tokio_tungstenite::connect_async(url).await.unwrap();

    let (outgoing, incoming) = ws_connection.split();

    let decoder = {
        let session_clone = gateway_session.clone();
        move |msg| {
            let session = session_clone.clone();
            async move {
                decode_message(msg, session).await
            }
        }
    };
    let mut decoded = Box::pin(incoming.then(decoder));

    let hello = match decoded.next().await
        .expect("Websocket should not close")
        .expect("Websocket should not error")
        {
            GatewayIncoming::Recv(GatewayRecvEvent::Hello(h)) => h,
            GatewayIncoming::Recv(_) => panic!("Received a gateway event other than Hello"),
            GatewayIncoming::Close(c) => panic!("Gateway closed immediately: {:?}", c)
        };

    let heartbeat_interval = hello.heartbeat_interval;

    let (other, heartbeat_events) = decoded
        .map(|item| {
            match item {
                Ok(GatewayIncoming::Recv(GatewayRecvEvent::Heartbeat(event))) => Either::Right(Into::<HeartbeatManagerInput>::into(event)),
                Ok(GatewayIncoming::Recv(GatewayRecvEvent::HeartbeatAck(event))) => Either::Right(Into::<HeartbeatManagerInput>::into(event)),
                res => Either::Left(res),
            }
        })
        .split_either();

    let encoded = Box::pin(outgoing.with(|event: GatewaySendEvent| async {
        Ok(event.into())
    }));

    let fan_in_encoded = FanInSink::new(encoded);

    let heartbeat_sink = fan_in_encoded.clone();

    let heartbeat_manager = HeartbeatManager::new(heartbeat_sink, heartbeat_events, Duration::from_millis(heartbeat_interval), gateway_session.clone());

    let final_stream = futures::stream::select(other, heartbeat_manager);


    return (fan_in_encoded, final_stream)
}

pub async fn resume_connection<NewInner>(session: Arc<RwLock<GatewaySession>>) 
    -> Result<NewInner, ()> 
where 
    NewInner: Sink<GatewaySendEvent, Error = GatewayError> + Stream<Item = Result<GatewayIncoming, GatewayError>> + Unpin
{
    let (mut inner, _response) = tokio_tungstenite::connect_async(resume_gateway_url).await.unwrap();

    let session_read = session.read().await;

    let resume_event = GatewaySendEvent::Resume(
        send::Resume { 
            token: session_read.token.to_string(), 
            session_id: session_read.session_id.to_string(), 
            seq: session_read.sequence_number 
        }
    );

    inner.send(resume_event.into()).await.unwrap();

    Ok(inner)
}

async fn on_send<Inner>(mut inner: Inner, item: GatewaySendEvent) -> Result<Inner, GatewayError> 
where 
    Inner: Sink<GatewaySendEvent, Error = GatewayError> + Stream<Item = Result<GatewayIncoming, GatewayError>> + Unpin
{
    match inner.send(item).await {
        Ok(_) => return Ok(inner),
        Err(e) => return  Err(e),
    }
}

async fn on_recv<Inner>(
    mut inner: Inner, 
    gateway_session: Arc<RwLock<GatewaySession>>
) -> Option<(Inner, GatewayRecvEvent)> 
where 
    Inner: Sink<GatewaySendEvent, Error = GatewayError> + Stream<Item = Result<GatewayIncoming, GatewayError>> + Unpin
{

    while let Some(res) = inner.next().await {
        match res {
            Ok(GatewayIncoming::Close(close_event)) => {
                if close_event.close_code.can_reconnect() {
                    match resume_connection(gateway_session.clone()).await {
                        Ok(new_inner) => {
                            inner = new_inner;
                            continue
                        },
                        Err(e) => {
                            eprintln!("Could not resume connection: {:?}", e);
                            return None;
                        }
                    }
                } else {
                    eprintln!("Could not reconnect. Reason: {:?}", close_event.reason);
                    return None;
                }
            }
            Ok(GatewayIncoming::Recv(recv_event)) => return Some((inner, recv_event)),
            Err(e) => {
                eprintln!("Gateway error: {:?}. Trying to resume connection.", e);
                match resume_connection(gateway_session.clone()).await {
                    Ok(new_inner) => {
                        inner = new_inner;
                        continue
                    },
                    Err(e) => {
                        eprintln!("Could not resume connection: {:?}", e);
                        return None;
                    }
                }
            }
        }
    };

    return None
}

pub async fn connect_websocket(url: &str) -> impl Sink<GatewaySendEvent> + Stream
{
    let gateway_session = Arc::new(RwLock::new(GatewaySession::new()));

    let on_recv = {
        let session_clone = gateway_session.clone();
        move |inner| {
            let session = session_clone.clone();
            async move {
                on_recv(inner, session).await
            }
        }
    };

    let (gateway_sink, gateway_stream) = create_connection(url, gateway_session.clone()).await;

    let duplex = Duplex::new(gateway_sink, gateway_stream);

    ReconnectManager::new(duplex, on_send, on_recv)
}





#[derive(Debug)]
pub struct Heartbeat {
    latest_sequence_number: Seq,
}

#[derive(Debug)]
pub struct HeartbeatAck {}

impl Heartbeat {
    pub fn new(latest_sequence_number: Seq) -> Self {
        Heartbeat { latest_sequence_number }
    }
}


pub enum HeartbeatManagerInput {
    Heartbeat(Heartbeat),
    HeartbeatAck(HeartbeatAck),
}

pin_project! {
    pub struct HeartbeatManager<Si, S, SinkItem> 
    where
        Si: Sink<SinkItem>,
    {
        #[pin]
        sink: Si,
        send_queue: VecDeque<SinkItem>,
        #[pin]
        incoming: S,
        #[pin]
        interval: Interval,
        ack_received: bool,

        session: Arc<RwLock<GatewaySession>>,
        #[pin]
        session_lock_fut: Option<Pin<Box<dyn Future<Output = OwnedRwLockReadGuard<GatewaySession>>>>>,

        _marker: PhantomData<SinkItem>,
    }
}

impl<Si, S, SinkItem> HeartbeatManager<Si, S, SinkItem> 
where 
    Si: Sink<SinkItem>
{
    pub fn new(sink: Si, incoming: S, heartbeat_interval: Duration, session: Arc<RwLock<GatewaySession>>) -> Self {
        HeartbeatManager { 
            sink,
            send_queue: VecDeque::new(),
            incoming, 
            interval: interval(heartbeat_interval), 
            ack_received: true,
            session,
            session_lock_fut: None,
            _marker: PhantomData
        }
    }

    fn get_latest_session_number(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Seq> {
        let mut this = self.project();

        if this.session_lock_fut.is_none() {
            *this.session_lock_fut = Some(this.session.clone().read_owned().boxed());
        }

        let fut = this.session_lock_fut.as_pin_mut().unwrap();
        let read_guard = ready!(fut.poll(cx));

        return Poll::Ready(read_guard.sequence_number)
    }
}

impl<Si, S, SinkItem, Item> Stream for HeartbeatManager<Si, S, SinkItem> 
where 
    S: Stream<Item = Item>,
    Si: Sink<SinkItem>,
    Item: Into<HeartbeatManagerInput>,
    Heartbeat: Into<SinkItem>,
    Si::Error: Into<GatewayError>,
{
    type Item = Result<GatewayIncoming, GatewayError>;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {

        loop{
            let mut this = self.as_mut().project();

            while !this.send_queue.is_empty() {
                ready!(this.sink.as_mut().poll_ready(cx));
                let sink_item = this.send_queue.pop_front().unwrap();
                match this.sink.as_mut().start_send(sink_item) {
                    Ok(()) => {},
                    Err(e) => return Poll::Ready(Some(Err(Into::<GatewayError>::into(e))))
                }
            }
            ready!(this.sink.as_mut().poll_flush(cx));

            let latest_sequence_number = ready!(self.as_mut().get_latest_session_number(cx));

            let mut this = self.as_mut().project();

            match this.incoming.as_mut().poll_next(cx) {
                Poll::Pending => {},
                Poll::Ready(Some(item)) => {
                    match item.into() {
                        HeartbeatManagerInput::HeartbeatAck(_ack) => {
                            *this.ack_received = true;
                        },
                        HeartbeatManagerInput::Heartbeat(_heartbeat) => {
                            this.interval.reset();
                            *this.ack_received = false;
                            this.send_queue.push_back(Heartbeat::new(latest_sequence_number).into());
                            continue
                        },
                    }
                }
                Poll::Ready(None) => return Poll::Ready(None),
            };

            match this.interval.poll_tick(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(_instant) => {
                    if *this.ack_received {
                        *this.ack_received = false;
                        this.send_queue.push_back(Heartbeat::new(latest_sequence_number).into());
                    } else {
                        return Poll::Ready(Some(Err(HeartbeatError {}.into())))
                    }
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use tokio::time::{advance, sleep, Duration};
    use tokio_stream::wrappers::ReceiverStream;

    #[tokio::test(start_paused = true)] // <-- IMPORTANT: we control time
    async fn test_heartbeat_manager_emits_heartbeats_and_handles_acks() {
        // incoming acks (empty at first)
        let (ack_tx, ack_rx) = tokio::sync::mpsc::channel(10);
        let incoming_stream = ReceiverStream::new(ack_rx);

        // create the heartbeat stream
        let mut hb_stream = HeartbeatManager::new(incoming_stream, Duration::from_secs(1));

        advance(Duration::from_millis(1)).await;
        let first = hb_stream.next().await.expect("expected heartbeat");
        println!("First: {:?}", &first);
        assert!(first.is_ok(), "first event should be Ok(Heartbeat)");

        //
        // ---- Send an ack BEFORE the next timeout ----
        //
        ack_tx.send(HeartbeatManagerInput::HeartbeatAck(HeartbeatAck {  })).await.unwrap();

        // Tick the clock so the next heartbeat is generated
        advance(Duration::from_secs(1)).await;

        let second = hb_stream.next().await.expect("expected second heartbeat");
        println!("Second: {:?}", &second);
        assert!(second.is_ok(), "second event should also be Ok(Heartbeat)");

        // Send heartbeat so the next heartbeat is generated early
        advance(Duration::from_millis(200)).await;
        ack_tx.send(HeartbeatManagerInput::Heartbeat(Heartbeat {  })).await.unwrap();

        let third = hb_stream.next().await.expect("expected third heartbeat");
        println!("Third: {:?}", &third);
        assert!(third.is_ok(), "third event should also be Ok(Heartbeat)");

        //
        // ---- Advance time long enough for a timeout event ----
        //
        advance(Duration::from_secs(2)).await;

        let fourth = hb_stream.next().await.expect("expected timeout event");
        println!("Fourth: {:?}", &fourth);
        assert!(fourth.is_err(), "expected Err(HeartbeatError)");
    }
}


