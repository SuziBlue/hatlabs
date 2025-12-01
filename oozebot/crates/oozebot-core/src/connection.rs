
use std::sync::Arc;

use futures::future::Either;
use futures::lock::Mutex;
use futures::task::ArcWake;
use futures_util::{SinkExt, StreamExt};
use oozebot_protocol::close_codes::GatewayCloseCode;
use oozebot_protocol::events::receive::{GatewayRecvEvent, Heartbeat, HeartbeatAck};
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::WebSocketStream;
use thiserror::Error;

use oozebot_protocol::events::send::{self, ClientProperties, GatewaySendEvent, Heartbeat, Identify, Resume};
use oozebot_protocol::intents::Intents;
use futures_util::stream::{SplitSink, SplitStream};
use serde_json;
use anyhow::Result;
use anyhow::anyhow;

use crate::protocols::{HeartbeatManager, HeartbeatManagerInput};
use crate::tasks::ConnectionTask;
use crate::streams::{HeartbeatProtocol, HeartbeatManager};





pub struct Session {
    sequence_number: Option<u64>,
    gateway_url: Option<String>,
    resume_gateway_url: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Session { 
            sequence_number: None, 
            gateway_url: None, 
            resume_gateway_url: None 
        }
    }
}

pub struct Connection {
    connection_state: RwLock<ConnectionState>,
    event_handler: EventHandler,
    event_sender: EventSender,
    session: RwLock<Session>,
}

#[derive(PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Identifying,
    Connected,
    Resuming,
}

pub enum ConnectionError {
    GatewayInitiatedClose(Option<GatewayCloseCode>),
    ClientInitiatedClose,
    Timeout,
    InternalChannelError,
    Other(String),
}

impl From<tokio::sync::broadcast::error::RecvError> for ConnectionError {
    fn from(_value: tokio::sync::broadcast::error::RecvError) -> Self {
        ConnectionError::InternalChannelError
    }
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for ConnectionError {
    fn from(_value: tokio::sync::broadcast::error::SendError<T>) -> Self {
        ConnectionError::InternalChannelError
    }
}

impl From<tungstenite::Error> for ConnectionError {
    fn from(value: tungstenite::Error) -> Self {
        ConnectionError::Other(value.to_string())
    }
}

impl Connection {
    pub fn new() -> Arc<Self> {
        Arc::new(Connection { 
            connection_state: ConnectionState::Disconnected.into(), 
            event_handler: EventHandler::new(), 
            event_sender: EventSender::new(), 
            session: Session::new().into(),
        })
    }

    pub async fn connect_to_websocket<S>(self: Arc<Self>, websocket_url: &str, input_stream: S) -> Result<()> 
    where 
        S: Stream<Item = GatewaySendEvent>
    {

        if *self.connection_state.read().await != ConnectionState::Disconnected {
            return Err(anyhow!("Cannot connect to gateway while already connected."))
        }

        self.session.write().await.gateway_url = Some(websocket_url.to_string());
        
        let (stream, _response) = tokio_tungstenite::connect_async(websocket_url).await?;

        let (write, read) = stream.split();

        let (texts, closes) = read
            .filter_map(|msg| {
                match msg {
                    tungstenite::Message::Text(text) => Some(Either::Left(text)),
                    tungstenite::Message::Close(frame) => Some(Either::Right(frame)),
                    _ => None,
                }
            })
            .split_either();

        let decoded = texts.filter_map(|text| {
            match serde_json::from_str::<GatewayRecvEvent>(text) {
                Ok(event) => Some(event),
                Err(e) => {
                    eprintln!("Failed to decode text: {:?}, Error: {:?}", text, e);
                    None
                }
            }
        });      

        let (heartbeat_events, other) = decoded
            .map(|event| {
                match event {
                    GatewayRecvEvent::HeartbeatAck(ack) => Either::Left(HeartbeatManagerInput::HeartbeatAck(())),
                    GatewayRecvEvent::Heartbeat(heartbeat) => Either::Left(HeartbeatManagerInput::Heartbeat(())),
                    _ => Either::Right(event),

                }
            })
            .split_either();

        let heartbeats = HeartbeatManager::new(heartbeat_events, heartbeat_interval)
            .map(|heartbeat| {
                match heartbeat {
                    Ok(h) => GatewaySendEvent::Heartbeat(send::Heartbeat),
                    Err(e)
                }
            });

        let output_stream = tokio_stream::StreamExt::merge(input_stream, heartbeats)

    }

    pub async fn send(&self, event: GatewaySendEvent) -> Result<()> {
        self.event_sender.send_event(event).await
    }

    async fn start_heartbeats(self: Arc<Self>) -> tokio::task::JoinHandle<Result<(), ConnectionError>> {

        tokio::spawn(async move {

            let interval = match self.event_handler.wait_for_event(|event| {
                match event {
                    GatewayRecvEvent::Hello(hello) => Some(hello),
                    _ => None, 
                }
            }).await {
                Ok(hello) => hello.heartbeat_interval,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {return Err(ConnectionError::Other("Event channel closed unexpectedly.".to_string()))}
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {return Err(ConnectionError::Other("Event channel lagged, may have missed ack.".to_string()))}
            };

            let sleep_duration = tokio::time::Duration::from_millis((interval as f64 * rand::random::<f64>()) as u64); 

            tokio::time::sleep(sleep_duration).await;

            let sleep_duration = tokio::time::Duration::from_millis(interval);

            loop {

                let payload = GatewaySendEvent::Heartbeat(Heartbeat { d: self.session.read().await.sequence_number });
                self.send(payload).await;

                let await_heartbeat_ack = {
                    self.event_handler.wait_for_event(|event| {
                            match event {
                                GatewayRecvEvent::HeartbeatAck(ack) => Some(ack),
                                _ => None, 
                            }
                    })
                };

                let ack_result = tokio::time::timeout(sleep_duration, await_heartbeat_ack).await;


                match ack_result {
                    Ok(Ok(ack)) => {
                        self.session.write().await.sequence_number = Some(ack.sequence_number);
                    },
                    Ok(Err(e)) => {
                        match e {
                            tokio::sync::broadcast::error::RecvError::Closed => {},
                            tokio::sync::broadcast::error::RecvError::Lagged(_) => return Err(ConnectionError::Other("Event channel lagged, may have missed ack.".to_string()))
                        }
                    },
                    Err(_) => {
                        return Err(ConnectionError::Timeout);
                    },
                }

            }
        })
    }
}

pub struct EventHandler{
    pub event_tx: tokio::sync::broadcast::Sender<GatewayRecvEvent>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (tx, _rx) = tokio::sync::broadcast::channel(10);
        Self { event_tx: tx }
    }
    pub fn start<S>(&self, gateway_stream: SplitStream<WebSocketStream<S>>) -> tokio::task::JoinHandle<Result<(), ConnectionError>>
    where
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
    {

        let tx_clone = self.event_tx.clone();

        tokio::spawn(async move {
            let mut stream = gateway_stream;

            while let Some(msg) = stream.next().await {
                match msg? {
                    tungstenite::Message::Text(text) => {
                        match serde_json::from_str(&text) {
                            Ok(event) => {
                                tx_clone.send(event)?;
                            },
                            Err(e) => {
                                eprintln!("Could not deserialize message: {}", text);
                                eprintln!("Error: {}", e);
                            },
                        };
                    },
                    tungstenite::Message::Close(close_frame) => {
                        let close_code = close_frame.map(|frame| u16::from(frame.code).into());
                        return Err(ConnectionError::GatewayInitiatedClose(close_code))
                    },
                    _ => {},
                }
            }

            return Err(ConnectionError::GatewayInitiatedClose(None))
        })
    }

    pub async fn wait_for_event<F, T>(&self, mut f: F) -> Result<T, tokio::sync::broadcast::error::RecvError>
    where
        F: FnMut(GatewayRecvEvent) -> Option<T>,
    {
        let mut rx = self.event_tx.subscribe();

        loop {
            match rx.recv().await {
                Ok(event) => {
                    if let Some(result) = f(event) {
                        return Ok(result);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }
}

pub struct EventSender{
    pub event_tx: tokio::sync::mpsc::Sender<GatewaySendEvent>,
    event_rx: Option<tokio::sync::mpsc::Receiver<GatewaySendEvent>>,
    close_tx: Option<tokio::sync::oneshot::Sender<()>>
}

impl EventSender {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(10);
        Self { 
            event_tx: tx,
            event_rx: Some(rx),
            close_tx: None,
        }
    }
    pub fn start<S>(&mut self, gateway_sink: SplitSink<WebSocketStream<S>, tungstenite::protocol::Message>) -> tokio::task::JoinHandle<Result<(), ConnectionError>> 
    where 
        S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
    {
        let mut rx = self.event_rx.take().expect("Receiver should exist.");

        let (close_tx, close_rx) = tokio::sync::oneshot::channel();

        self.close_tx = Some(close_tx);

        tokio::spawn(async move {
            let mut sink = gateway_sink;

            while let Some(event) = rx.recv().await {
                match serde_json::to_string(&event).map(|json| tungstenite::Message::Text(json.into())) {
                    Ok(message) => sink.send(message).await?,
                    Err(e) => {
                        eprintln!("Could not serialize message: {:?}", event);
                        eprintln!("Error: {}", e);
                    },
                } 
            }

            sink.send(tungstenite::Message::Close(None));
            sink.close();

            Ok(())
        })
    }
    pub async fn send_event(&self, event: GatewaySendEvent) -> Result<()> {
        self.event_tx.send(event).await?;
        Ok(())
    }
}




#[derive(Debug, Error)]
pub enum HeartbeatError {
    #[error("Channel receive error: {0}")]
    RecvError(#[from] tokio::sync::broadcast::error::RecvError),

    #[error("Missing heartbeat ACK within timeout")]
    Timeout,

    #[error("Unexpected message received")]
    UnexpectedMessage,

    #[error("Internal error: {0}")]
    Internal(String),
}

async fn heartbeat_handler(event_handler: EventHandler, event_sender: EventSender, session: Arc<RwLock<Session>>) -> Result<, ConnectionError> {
    let interval = match event_handler.wait_for_event(|event| {
        match event {
            GatewayRecvEvent::Hello(hello) => Some(hello),
            _ => None, 
        }
    }).await {
        Ok(hello) => hello.heartbeat_interval,
        Err(tokio::sync::broadcast::error::RecvError::Closed) => {return Err(ConnectionError::Other("Event channel closed unexpectedly.".to_string()))}
        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {return Err(ConnectionError::Other("Event channel lagged, may have missed ack.".to_string()))}
    };

    let sleep_duration = tokio::time::Duration::from_millis((interval as f64 * rand::random::<f64>()) as u64); 

    tokio::time::sleep(sleep_duration).await;

    let sleep_duration = tokio::time::Duration::from_millis(interval);

    loop {

        let payload = GatewaySendEvent::Heartbeat(Heartbeat { d: session.read().await.sequence_number });
        event_sender.send_event(payload).await;

        let await_heartbeat_ack = {
            event_handler.wait_for_event(|event| {
                    match event {
                        GatewayRecvEvent::HeartbeatAck(ack) => Some(ack),
                        _ => None, 
                    }
            })
        };

        let ack_result = tokio::time::timeout(sleep_duration, await_heartbeat_ack).await;


        match ack_result {
            Ok(Ok(ack)) => {
                session
                    .write()
                    .await
                    .sequence_number = Some(ack.sequence_number);
            },
            Ok(Err(e)) => {
                match e {
                    tokio::sync::broadcast::error::RecvError::Closed => {},
                    tokio::sync::broadcast::error::RecvError::Lagged(_) => return Err(ConnectionError::Other("Event channel lagged, may have missed ack.".to_string()))
                }
            },
            Err(_) => {
                return Err(ConnectionError::Timeout);
            },
        }

    }
}



pub struct DiscordHeartbeatProtocol {}

pub struct DiscordHeartbeatError;


impl HeartbeatProtocol for DiscordHeartbeatProtocol {

    type SinkItem = GatewaySendEvent;
    type StreamItem = GatewayRecvEvent;
    type Heartbeat = Heartbeat;
    type HeartbeatAck = HeartbeatAck;
    type HeartbeatError = DiscordHeartbeatError;


    fn start<Source>(self, 
        inner: Source, 
        sink_channel: futures::channel::mpsc::Receiver<Self::SinkItem>, 
        stream_channel: futures::channel::mpsc::Sender<std::result::Result<Self::StreamItem, Self::HeartbeatError>>
    )
    where 
        Source: futures::Sink<Self::SinkItem> + futures::Stream<Item = Self::StreamItem> 
    {
        


        tokio::spawn(async move {
            

            let (inner_stream, inner_sink) = inner.split();
        })
    }
}





