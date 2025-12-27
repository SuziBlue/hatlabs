
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Context;
use std::{task::Poll, time::Duration};

use futures::{future, ready, FutureExt, Sink, SinkExt, Stream, StreamExt, TryStreamExt};
use futures_util::future::Either;
use oozebot_protocol::events::receive::{self, GatewayCloseEvent, GatewayIncoming, GatewayRecvEvent};
use oozebot_protocol::events::send::{self, GatewayOutgoing, GatewaySendEvent};
use oozebot_protocol::{GatewayError, HeartbeatError, RawGatewayPayload};
use pin_project_lite::pin_project;
use tokio::sync::{OwnedRwLockReadGuard, RwLock};
use tokio::time::{interval, Interval};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{self, Message};

use crate::streams::{CloseEvent, Duplex, FanInSink, SessionHandler, StreamExtSplit};



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

impl From<Heartbeat> for Either<GatewaySendEvent, GatewayCloseEvent> {
    fn from(value: Heartbeat) -> Self {
        Either::Left(value.into())
    }
}


type Seq = Option<u64>;

pub struct GatewaySession {
    sequence_number: Seq,
    resume_gateway_url: Option<String>,
    token: String,
    session_id: Option<String>,
}

impl GatewaySession {
    fn new(token: String) -> GatewaySession {
        GatewaySession {
            sequence_number: None,
            resume_gateway_url: None,
            token,
            session_id: None,
        }
    }
}

async fn decode_message(msg: Result<Message, tungstenite::Error>, gateway_session: Arc<RwLock<GatewaySession>>) -> Result<Either<GatewayRecvEvent, GatewayCloseEvent>, GatewayError> {

    match msg {
        Ok(Message::Text(text)) => {
            match serde_json::from_str::<RawGatewayPayload>(&text) {
                Ok(raw) => {
                    if raw.s.is_some() {
                        let mut session_write = gateway_session.write().await;
                        session_write.sequence_number = raw.s;
                    }
                    match TryInto::<GatewayRecvEvent>::try_into(raw) {
                        Ok(event) => return Ok(Either::Left(event)),
                        Err(e) => return Err(Into::<GatewayError>::into(e))
                    }
                },
                Err(e) => return Err(Into::<GatewayError>::into(e))
            }
        },
        Ok(Message::Close(maybe_frame)) => {
            let close_event = Into::<GatewayCloseEvent>::into(maybe_frame);
            return Ok(Either::Right(close_event))
        },
        Ok(msg) => {panic!("Received an unexpected message type from Discord gateway: {:?}", msg)},
        Err(e) => return Err(Into::<GatewayError>::into(e))
    };
}

pub async fn create_connection(
    ws_connection: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, 
    gateway_session: Arc<RwLock<GatewaySession>>
) -> (Pin<Box<dyn Sink<Either<GatewaySendEvent, GatewayCloseEvent>, Error = GatewayError>>>, Pin<Box<dyn Stream<Item = Result<Either<GatewayRecvEvent, GatewayCloseEvent>, GatewayError>>>>) 
{

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
            Either::Left(GatewayRecvEvent::Hello(h)) => h,
            Either::Left(_) => panic!("Received a gateway event other than Hello"),
            Either::Right(c) => panic!("Gateway closed immediately: {:?}", c)
        };

    let heartbeat_interval = hello.heartbeat_interval;

    let (other, heartbeat_events) = decoded
        .map(|item| {
            match item {
                Ok(Either::Left(GatewayRecvEvent::Heartbeat(event))) => Either::Right(Into::<HeartbeatManagerInput>::into(event)),
                Ok(Either::Left(GatewayRecvEvent::HeartbeatAck(event))) => Either::Right(Into::<HeartbeatManagerInput>::into(event)),
                res => Either::Left(res),
            }
        })
        .split_either();

    let encoded = Box::pin(outgoing.with(|event: Either<GatewaySendEvent, GatewayCloseEvent>| async {
        match event {
            Either::Left(send) => Ok(send.into()),
            Either::Right(close) => Ok(close.into()),
        }
    }));

    let fan_in_encoded = FanInSink::new(encoded);

    let heartbeat_sink = fan_in_encoded.clone();

    let heartbeat_manager = HeartbeatManager::new(heartbeat_sink, heartbeat_events, Duration::from_millis(heartbeat_interval), gateway_session.clone())
        .map(|item| {
            match item {
                Ok(GatewayIncoming::Recv(recv)) => Ok(Either::Left(recv)),
                Ok(GatewayIncoming::Close(close)) => Ok(Either::Right(close)),
                Err(e) => Err(e)
            }
        });

    let final_stream = futures::stream::select(other, heartbeat_manager);

    return (Box::pin(fan_in_encoded), Box::pin(final_stream))
}

pub async fn resume_connection(session: Arc<RwLock<GatewaySession>>) -> Option<Duplex<Pin<Box<dyn Sink<Either<GatewaySendEvent, GatewayCloseEvent>, Error = GatewayError>>>, Pin<Box<dyn Stream<Item = Result<Either<GatewayRecvEvent, GatewayCloseEvent>, GatewayError>>>>, Either<GatewaySendEvent, GatewayCloseEvent>>> 
{

    let session_read = session.read().await;

    let request = session_read.resume_gateway_url.clone()
        .expect("No resume url")
        .into_client_request()
        .expect("Invalid resume url");

    let (ws_connection, _response) = tokio_tungstenite::connect_async(request).await.unwrap();

    let (mut sink, stream) = create_connection(ws_connection, session.clone()).await;



    let resume_event = Either::Left(GatewaySendEvent::Resume(
        send::Resume { 
            token: session_read.token.clone(),
            session_id: session_read.session_id.clone().expect("No session id"),
            seq: session_read.sequence_number.expect("No sequence number"),
        }
    ));

    sink.send(resume_event.into()).await.unwrap();

    let inner = Duplex::new(sink, stream);
    Some(inner)
}

pub async fn connect_websocket(url: &str, token: &str)
{
    let gateway_session = Arc::new(RwLock::new(GatewaySession::new(token.to_string())));

    let (ws_connection, _response) = tokio_tungstenite::connect_async(url).await.unwrap();

    let (gateway_sink, gateway_stream) = create_connection(ws_connection, gateway_session.clone()).await;

    let duplex = Duplex::new(gateway_sink, gateway_stream);

    let session_manager = SessionHandler::from_sink(duplex, move |mut inner, event| {

        let gateway_session = gateway_session.clone();

        async move {
            match event {
                Some(CloseEvent::Internal(close)) => {
                    let _ = inner.send(Either::Right(close)).await; 
                    let _ = inner.close().await;
                    return None
                }
                Some(CloseEvent::External(close)) => {
                    if let Some(code) = close.close_code {
                        if code.can_reconnect() {
                            let new_inner = resume_connection(gateway_session.clone()).await.map(Box::pin);
                            return new_inner
                        }
                    }

                    return None
                }
                None => {
                    let new_inner = resume_connection(gateway_session.clone()).await.map(Box::pin);
                    return new_inner
                }
            }
        }
    });

    todo!()
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


