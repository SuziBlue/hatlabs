
use std::{task::Poll, time::Duration};

use futures::future::Either;
use futures::{Sink, SinkExt, Stream, StreamExt};
use oozebot_protocol::close_codes::GatewayCloseCode;
use oozebot_protocol::events::receive::{self, GatewayRecvEvent};
use oozebot_protocol::events::send::{self, GatewaySendEvent};
use pin_project_lite::pin_project;
use tokio::time::{interval, Interval};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::streams::{ReconnectManager, StreamExtSplit};



impl From<receive::Heartbeat> for Heartbeat {
    fn from(_value: receive::Heartbeat) -> Self {
        Heartbeat {  }
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

pub async fn create_connection(url: &str) -> impl Sink<GatewaySendEvent> + Stream {
    let (mut ws_connection, _response) = tokio_tungstenite::connect_async(url).await.unwrap();

    let hello = match ws_connection.next().await
        .expect("Websocket should not close")
        .expect("Websocket should not error")
        .into_text()
        .expect("Message should be json")
        .try_into()
        .expect("Message should be deserializable") {
            GatewayRecvEvent::Hello(h) => h,
            _ => panic!("Received a gateway event other than Hello")
        };
    let heartbeat_interval = hello.heartbeat_interval;

    let (outgoing, incoming) = ws_connection.split();

    let (other, heartbeat_events) = incoming
        .map(|item| {
            match item {
                Ok(Message::Text(text)) => {
                    let event = TryInto::<GatewayRecvEvent>::try_into(text).expect("Could not deserialize text");
                    Ok(event)
                },
                Ok(_) => panic!("Received an unknown message type"),
                Err(e) => Err(e),
            }
        })
        .map(|item| {
            match item {
                Ok(GatewayRecvEvent::Heartbeat(event)) => Either::Right(Into::<HeartbeatManagerInput>::into(event)),
                Ok(GatewayRecvEvent::HeartbeatAck(event)) => Either::Right(Into::<HeartbeatManagerInput>::into(event)),
                res => Either::Left(res),
            }
        })
        .split_either();

    let heartbeat_manager = HeartbeatManager::new(heartbeat_events, Duration::from_millis(heartbeat_interval));

    let (heartbeat_errors, heartbeats) = heartbeat_manager.map(|item| {
        match item {
            Ok(heartbeat) => Either::Right(heartbeat),
            Err(error) => Either::Left(error),
        }
    })
    .split_either();

    todo!();
}

pub fn should_reconnect(close_frame: Option<CloseFrame>) -> bool {
    if close_frame.is_none() {
        return true
    }
    if let Ok(close_code) = GatewayCloseCode::try_from(close_frame.expect("Close frame is not None").code) {
        return close_code.can_reconnect()
    } else {
        return false
    }
}

pub async fn resume_connection(token: &str, session_id: &str, resume_gateway_url: &str, sequence_number: u64) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, ()> {
    let (mut inner, _response) = tokio_tungstenite::connect_async(resume_gateway_url).await.unwrap();

    let resume_event = GatewaySendEvent::Resume(
        send::Resume { 
            token: token.to_string(), 
            session_id: session_id.to_string(), 
            seq: sequence_number 
        }
    );

    inner.send(resume_event.into()).await.unwrap();

    Ok(inner)
}

pub async fn connect_websocket(url: &str) -> impl Sink<Message> + Stream
{

    let (inner, _response) = tokio_tungstenite::connect_async(url).await.unwrap();

    let on_send = |mut inner: WebSocketStream<MaybeTlsStream<TcpStream>>, item: Message| async {
        match inner.send(item).await {
            Ok(_) => return Ok(inner),
            Err(e) => return  Err(e),
        }
    };


    let url_clone = url.clone();

    let on_recv = move |mut inner: WebSocketStream<MaybeTlsStream<TcpStream>>| {
        async move {
            while let Some(res) = inner.next().await {
                match res {
                    Ok(Message::Close(maybe_closeframe)) => {
                        if should_reconnect(maybe_closeframe) {
                            let new_inner = resume_connection().await;
                            inner = new_inner;
                            continue
                        } else {
                            return None;
                        }
                    }
                    Ok(msg) => return Some((inner, msg)),
                    Err(_e) => {
                        return None;
                    }
                }
            };

            return None
        }
    };


    ReconnectManager::new(inner, on_send, on_recv)
}





#[derive(Debug)]
pub struct Heartbeat {}

#[derive(Debug)]
pub struct HeartbeatAck {}

impl Heartbeat {
    pub fn new() -> Self {
        Heartbeat {  }
    }
}

#[derive(Debug)]
pub struct HeartbeatError {}

pub enum HeartbeatManagerInput {
    Heartbeat(Heartbeat),
    HeartbeatAck(HeartbeatAck),
}

pin_project! {
    pub struct HeartbeatManager<S> {
        #[pin]
        incoming: S,
        #[pin]
        interval: Interval,
        ack_received: bool,
    }
}

impl<S> HeartbeatManager<S> {
    pub fn new(incoming: S, heartbeat_interval: Duration) -> Self {
        HeartbeatManager { 
            incoming, 
            interval: interval(heartbeat_interval), 
            ack_received: true 
        }
    }
}

impl<S, Item> Stream for HeartbeatManager<S> 
where 
    S: Stream<Item = Item>,
    Item: Into<HeartbeatManagerInput>,
{
    type Item = Result<Heartbeat, HeartbeatError>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            match this.incoming.as_mut().poll_next(cx) {
                Poll::Pending => break,
                Poll::Ready(Some(item)) => {
                    match item.into() {
                        HeartbeatManagerInput::HeartbeatAck(_ack) => {
                            *this.ack_received = true;
                            continue
                        },
                        HeartbeatManagerInput::Heartbeat(_heartbeat) => {
                            *this.ack_received = false;
                            this.interval.reset();
                            return Poll::Ready(Some(Ok(Heartbeat {  })))
                        },
                    }
                }
                Poll::Ready(None) => return Poll::Ready(None),
            };
        }

        match this.interval.poll_tick(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(_instant) => {
                if *this.ack_received {
                    *this.ack_received = false;
                    return Poll::Ready(Some(Ok(Heartbeat {})))
                } else {
                    return Poll::Ready(Some(Err(HeartbeatError {})))
                }
            }
        };
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


