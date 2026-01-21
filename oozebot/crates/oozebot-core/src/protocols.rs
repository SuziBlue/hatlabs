
use std::collections::VecDeque;
use std::pin::Pin;
use std::{task::Poll, time::Duration};

use either::Either;
use futures::{ready, FutureExt, Sink, SinkExt, Stream, StreamExt};
use oozebot_protocol::close_codes::GatewayCloseEvent;
use oozebot_protocol::events::receive::{self, GatewayIncoming, GatewayRecvEvent};
use oozebot_protocol::events::send::{self, ClientProperties, GatewayOutgoing, GatewaySendEvent, Identify};
use oozebot_protocol::intents::Intents;
use oozebot_protocol::{GatewayError, HeartbeatError, RawGatewayPayload, WithSequenceNumber};
use pin_project_lite::pin_project;
use tokio::time::{interval, Interval};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::{self, Message};
use tracing::info;

use crate::streams::{Duplex, FanInSink};



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
        let heartbeat = send::Heartbeat(value.latest_sequence_number);
        GatewaySendEvent::Heartbeat(heartbeat)
    }
}

impl From<Heartbeat> for Either<GatewaySendEvent, GatewayCloseEvent> {
    fn from(value: Heartbeat) -> Self {
        Either::Left(value.into())
    }
}


type Seq = Option<u64>;

fn decode_message(msg: Result<Message, tungstenite::Error>) -> Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError> {

    match msg {
        Ok(Message::Text(text)) => {
            info!(target: "gateway.recv", json = %text, "received gateway message");

            return serde_json::from_str::<RawGatewayPayload>(&text)        
                .map_err(Into::into)
                .and_then(TryInto::try_into)
        },
        Ok(Message::Close(maybe_frame)) => {
            let close_event = Into::<GatewayCloseEvent>::into(maybe_frame);
            return Ok(close_event.into())
        },
        Ok(msg) => {panic!("Received an unexpected message type from Discord gateway: {:?}", msg)},
        Err(e) => return Err(Into::<GatewayError>::into(e))
    };
}

pub async fn create_connection(
    ws_connection: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, 
) -> (Pin<Box<dyn Sink<GatewayOutgoing, Error = GatewayError>>>, Pin<Box<dyn Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>>>>) 
{

    let (outgoing, incoming) = ws_connection.split();

    let mut decoded = Box::pin(incoming.map(decode_message));

    let hello = match decoded.next().await
        .expect("Websocket should not close")
        .expect("Websocket should not error")
        {
            GatewayIncoming::Recv(recv) => {
                match recv.into_inner() {
                    GatewayRecvEvent::Hello(h) => h,
                    _ => panic!("Received a gateway event other than Hello")
                }
            },
            GatewayIncoming::Close(c) => panic!("Gateway closed immediately: {:?}", c)
        };

    let heartbeat_interval = hello.heartbeat_interval;


    let encoded = Box::pin(outgoing.with(|event: GatewayOutgoing| async move {
        let msg: Message = event.into();

        match &msg {
            Message::Text(text) => {
                info!(json = %text, "sending websocket text message");
            }
            Message::Close(frame) => {
                info!(?frame, "sending websocket close");
            }
            _ => {
                info!(kind = ?msg, "sending websocket control frame");
            }
        }

        Ok(msg)
    }));


    let fan_in_encoded = FanInSink::new(encoded);

    let heartbeat_sink = fan_in_encoded.clone();

    let heartbeat_manager = HeartbeatManager::new(heartbeat_sink, decoded, Duration::from_millis(heartbeat_interval));

    return (Box::pin(fan_in_encoded), Box::pin(heartbeat_manager))
}

pub async fn resume_connection(
    resume_gateway_url: &str, 
    token: &str, 
    session_id: &str, 
    sequence_number: u64
) -> Option<(Pin<Box<dyn Sink<GatewayOutgoing, Error = GatewayError>>>, Pin<Box<dyn Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>>>>)> 
{

    let request = resume_gateway_url
        .into_client_request()
        .expect("Invalid resume url");

    let (ws_connection, _response) = tokio_tungstenite::connect_async(request).await.unwrap();

    let (mut sink, stream) = create_connection(ws_connection).await;



    let resume_event = GatewayOutgoing::Send(GatewaySendEvent::Resume(
        send::Resume { 
            token: token.to_string(),
            session_id: session_id.to_string(),
            seq: sequence_number,
        }
    ));

    sink.send(resume_event.into()).await.unwrap();

    Some((sink, stream))
}

pub async fn connect_websocket(url: &str, token: &str) -> impl Sink<GatewayOutgoing, Error = GatewayError> + Stream<Item = Result<GatewayRecvEvent, GatewayError>>
{
    let (ws_connection, _response) = tokio_tungstenite::connect_async(url).await.unwrap();

    let (mut gateway_sink, mut gateway_stream) = create_connection(ws_connection).await;

    let identify = Identify {
        token: token.to_string(),
        properties: ClientProperties {
            os: "Linux".to_string(),
            browser: "my_library".to_string(),
            device: "my_library".to_string(),
        },
        compress: None,
        large_threshold: None,
        shard: None,
        presence: None,
        intents: Intents::GUILDS.bits(),
    };

    gateway_sink.send(GatewayOutgoing::Send(GatewaySendEvent::Identify(identify))).await.unwrap();

    let mut sequence_number = None;
    let mut resume_gateway_url = "".to_string();
    let mut session_id = "".to_string();

    match gateway_stream.next().await {
        Some(Ok(GatewayIncoming::Recv(recv))) => {
            sequence_number = recv.sequence_number();
            match recv.into_inner() {
                GatewayRecvEvent::Ready(ready) => {
                    resume_gateway_url = ready.resume_gateway_url;
                    session_id = ready.session_id;
                },
                _ => panic!("Unexpected event."),
            }
        },
        Some(Ok(GatewayIncoming::Close(close))) => panic!("Gateway closed during identify {:?}", close),
        Some(Err(e)) => panic!("Error occured during identify: {:?}", e),
        None => panic!("Stream ended unexpectedly")
    }

    let close_manager = CloseManager::new(Duplex::new(gateway_sink, gateway_stream), &resume_gateway_url, &token, &session_id, sequence_number.unwrap());

    return close_manager
}


type Inner = Duplex<Pin<Box<dyn Sink<GatewayOutgoing, Error = GatewayError>>>, Pin<Box<dyn Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>>>>, GatewayOutgoing>;

#[pin_project::pin_project()]
pub struct CloseManager {
    #[pin]
    inner: Inner,
    inner_fut: Option<Pin<Box<dyn Future<Output = Option<Inner>>>>>,

    resume_gateway_url: String,
    token: String,
    session_id: String,
    sequence_number: u64,
}

impl CloseManager {
    pub fn new(inner: Inner, resume_gateway_url: &str, token: &str, session_id: &str, sequence_number: u64) -> Self {
        Self { 
            inner, 
            inner_fut: None, 
            resume_gateway_url: resume_gateway_url.to_string(), 
            token: token.to_string(), 
            session_id: session_id.to_string(), 
            sequence_number
        }
    }
}

impl Stream for CloseManager 
{
    type Item = Result<GatewayRecvEvent, GatewayError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            
            if let Some(fut) = this.inner_fut {
                if let Some(new_inner) = ready!(fut.poll_unpin(cx)) {
                    *this.inner = new_inner;
                    *this.inner_fut = None;
                } else {
                    return Poll::Ready(Some(Err(GatewayError::ResumeError)))
                };
            }

            match ready!(this.inner.as_mut().poll_next(cx)) {
                None => return Poll::Ready(None),
                Some(Ok(GatewayIncoming::Recv(recv))) => {
                    if let Some(sequence_number) = recv.sequence_number() {
                        *this.sequence_number = sequence_number;
                    }
                    return Poll::Ready(Some(Ok(recv.into_inner())))
                }
                Some(Ok(GatewayIncoming::Close(close))) => {
                    if close.can_reconnect() {
                        let sequence_number = *this.sequence_number;
                        let resume_gateway_url = this.resume_gateway_url.clone();
                        let token = this.token.clone();
                        let session_id = this.session_id.clone();
                        *this.inner_fut = Some(Box::pin(async move {
                            resume_connection(&resume_gateway_url, &token, &session_id.clone(), sequence_number)
                                .await
                                .map(|(sink, stream)| {
                                    Duplex::new(sink, stream)
                                })
                        }));
                        continue
                    } else {
                        return Poll::Ready(Some(Err(GatewayError::Closed(close.close_code.unwrap()))))
                    }
                }
                Some(Err(GatewayError::HeartbeatError(_))) => {
                    let sequence_number = *this.sequence_number;
                    let resume_gateway_url = this.resume_gateway_url.clone();
                    let token = this.token.clone();
                    let session_id = this.session_id.clone();
                    *this.inner_fut = Some(Box::pin(async move {
                        resume_connection(&resume_gateway_url, &token, &session_id.clone(), sequence_number)
                            .await
                            .map(|(sink, stream)| {
                                Duplex::new(sink, stream)
                            })
                    }));
                    continue
                }
                Some(Err(e)) => return Poll::Ready(Some(Err(e)))
            }
        }
    }
}

impl Sink<GatewayOutgoing> for CloseManager {
    type Error = GatewayError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: GatewayOutgoing) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
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

struct MaybeHeartbeatManagerInput(Option<HeartbeatManagerInput>);

impl From<GatewayRecvEvent> for MaybeHeartbeatManagerInput {
    fn from(value: GatewayRecvEvent) -> Self {
        let opt = match value {
            GatewayRecvEvent::Heartbeat(heartbeat) => Some(HeartbeatManagerInput::Heartbeat(heartbeat.into())),
            GatewayRecvEvent::HeartbeatAck(ack) => Some(HeartbeatManagerInput::HeartbeatAck(ack.into())),
            _ => None
        };
        MaybeHeartbeatManagerInput(opt)
    }
}

pin_project! {
    pub struct HeartbeatManager<Si, S> 
    where
        Si: Sink<GatewayOutgoing>,
    {
        #[pin]
        sink: Si,
        send_queue: VecDeque<GatewayOutgoing>,
        #[pin]
        incoming: S,
        #[pin]
        interval: Interval,
        ack_received: bool,
        sequence_number: Seq,
    }
}

impl<Si, S> HeartbeatManager<Si, S> 
where 
    Si: Sink<GatewayOutgoing>
{
    pub fn new(sink: Si, incoming: S, heartbeat_interval: Duration) -> Self {
        HeartbeatManager { 
            sink,
            send_queue: VecDeque::new(),
            incoming, 
            interval: interval(heartbeat_interval), 
            ack_received: true,
            sequence_number: None,
        }
    }
}

impl<Si, S> Stream for HeartbeatManager<Si, S> 
where 
    S: Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>>,
    Si: Sink<GatewayOutgoing>,
    GatewayError: From<Si::Error>,
{
    type Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {

        loop{
            let mut this = self.as_mut().project();

            while !this.send_queue.is_empty() {
                ready!(this.sink.as_mut().poll_ready(cx))?;
                let sink_item = this.send_queue.pop_front().unwrap();
                match this.sink.as_mut().start_send(sink_item) {
                    Ok(()) => {},
                    Err(e) => return Poll::Ready(Some(Err(Into::<GatewayError>::into(e))))
                }
            }
            ready!(this.sink.as_mut().poll_ready(cx))?;
            ready!(this.sink.as_mut().poll_flush(cx))?;

            let mut this = self.as_mut().project();

            match this.incoming.as_mut().poll_next(cx) {
                Poll::Pending => {},
                Poll::Ready(Some(item)) => {
                    match item {
                        Ok(GatewayIncoming::Recv(recv)) => {
                            *this.sequence_number = recv.sequence_number();
                            match recv.inner_ref() {
                                GatewayRecvEvent::HeartbeatAck(_ack) => {
                                    *this.ack_received = true;
                                },
                                GatewayRecvEvent::Heartbeat(_heartbeat) => {
                                    this.interval.reset();
                                    *this.ack_received = false;
                                    this.send_queue.push_back(GatewayOutgoing::Send(Heartbeat::new(*this.sequence_number).into()));
                                    continue
                                },
                                _ => {
                                    return Poll::Ready(Some(Ok(GatewayIncoming::Recv(recv))))
                                }
                            }
                        },
                        Ok(GatewayIncoming::Close(c)) => return Poll::Ready(Some(Ok(GatewayIncoming::Close(c)))),
                        Err(e) => return Poll::Ready(Some(Err(e))),
                    }
                }
                Poll::Ready(None) => return Poll::Ready(None),
            };

            match this.interval.poll_tick(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(_instant) => {
                    if *this.ack_received {
                        *this.ack_received = false;
                        this.send_queue.push_back(GatewayOutgoing::Send(Heartbeat::new(*this.sequence_number).into()));
                    } else {
                        return Poll::Ready(Some(Err(HeartbeatError {}.into())))
                    }
                }
            };
        }
    }
}


#[tokio::test]
async fn connect_to_gateway() {

    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_test_writer()
        .try_init();

    rustls::crypto::ring::default_provider().install_default().expect("Failed to install default rustls crypto provider");

    let gateway_url = reqwest::get("https://discord.com/api/v10/gateway")
        .await.unwrap()
        .json::<serde_json::Value>()
        .await.unwrap()["url"]
        .as_str().unwrap()
        .to_string();

    dotenvy::dotenv().unwrap();
    let token: &str = &std::env::var("GATEWAY_TOKEN").expect("Could not find token.");

    let mut connection = connect_websocket(&gateway_url, token).await;

    println!("Connection established");

    connection.close().await.expect("Failed to close connection");

    println!("Connection closed");
}




