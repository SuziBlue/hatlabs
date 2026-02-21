
use std::collections::VecDeque;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use std::{task::Poll, time::Duration};

use either::Either;
use futures::lock::Mutex;
use futures::{ready, FutureExt, Sink, SinkExt, Stream, StreamExt};
use oozebot_protocol::close_codes::GatewayCloseEvent;
use oozebot_protocol::events::receive::{self, GatewayIncoming, GatewayRecvEvent, GuildCreateEvent };
use oozebot_protocol::events::send::{self, ClientProperties, GatewayOutgoing, GatewaySendEvent, Identify};
use oozebot_protocol::intents::Intents;
use oozebot_protocol::resources::guild::{Guild, GuildCreate};
use oozebot_protocol::{BetterSerdeError, GatewayError, HeartbeatError, RawGatewayPayload, WithSequenceNumber};
use pin_project_lite::pin_project;
use tokio::net::TcpStream;
use tokio::time::{interval, Interval};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
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

pub type WebSocketConnection = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub async fn connect_websocket(url: &str) -> WebSocketConnection
{
    let (ws_connection, _response) = tokio_tungstenite::connect_async(url).await.unwrap();

    ws_connection
}

pub type ConnectionDecoded = Duplex<
    Pin<Box<dyn Sink<GatewayOutgoing, Error = GatewayError>>>, 
    Pin<Box<dyn Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>>>>,
>;

trait DecodeConnection<E>: Sink<Message, Error = E> + Stream<Item = Result<Message, E>> + Sized + 'static 
where 
    GatewayError: From<E>,
{
    fn decode_connection(
        self,
    ) -> ConnectionDecoded;
}

impl<Connection> DecodeConnection<tokio_tungstenite::tungstenite::Error> for Connection 
where 
    Connection: Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Sink<Message, Error = tokio_tungstenite::tungstenite::Error> + 'static,
{
    fn decode_connection(
            self,
        ) -> ConnectionDecoded {
        
        let (outgoing, incoming) = self.split();

        let decoded = Box::pin(incoming.map(|msg| {
            match msg {
                Ok(Message::Text(text)) => {
                    return serde_json::from_str::<RawGatewayPayload>(&text)        
                        .map_err(|e| <(serde_json::Error, &str) as Into::<BetterSerdeError>>::into((e, &text.to_string())).into())
                        .and_then(TryInto::try_into)
                },
                Ok(Message::Close(maybe_frame)) => {
                    let close_event = Into::<GatewayCloseEvent>::into(maybe_frame);
                    return Ok(close_event.into())
                },
                Ok(msg) => {panic!("Received an unexpected message type from Discord gateway: {:?}", msg)},
                Err(e) => return Err(Into::<GatewayError>::into(e))
            };
        }));

        let encoded = Box::pin(outgoing.with(|event: GatewayOutgoing| async move {
            let msg: Message = event.into();

            Ok(msg)
        }));

        return Duplex::new(encoded, decoded)
    }
}


pub type GatewayConnectionConnected = Duplex<
    Pin<Box<dyn Sink<GatewayOutgoing, Error = GatewayError>>>, 
    Pin<Box<dyn Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>>>>,
>;

trait CreateConnection: 
    Sink<GatewayOutgoing, Error = GatewayError> + 
    Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>> + 
    Sized + 
    'static 
{
    async fn create_connection(
        self, 
    ) -> GatewayConnectionConnected
    {
        let (encoded, mut decoded) = self.split();

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

        let fan_in_encoded = FanInSink::new(encoded);

        let heartbeat_sink = fan_in_encoded.clone();

        let heartbeat_manager = HeartbeatManager::new(heartbeat_sink, decoded, Duration::from_millis(heartbeat_interval));

        return Duplex::new(Box::pin(fan_in_encoded), Box::pin(heartbeat_manager))
    }
}

impl CreateConnection for ConnectionDecoded {}

pub async fn resume_connection(
    resume_gateway_url: &str, 
    token: &str, 
    session_id: &str, 
    sequence_number: u64
) -> Option<GatewayConnectionConnected> 
{

    let request = resume_gateway_url
        .into_client_request()
        .expect("Invalid resume url");

    let (ws_connection, _response) = tokio_tungstenite::connect_async(request).await.unwrap();

    let mut connection = ws_connection
        .decode_connection()
        .create_connection()
        .await;



    let resume_event = GatewayOutgoing::Send(GatewaySendEvent::Resume(
        send::Resume { 
            token: token.to_string(),
            session_id: session_id.to_string(),
            seq: sequence_number,
        }
    ));

    connection.send(resume_event.into()).await.unwrap();

    Some(connection)
}

pub type GatewayConnectionIdentified<C> = CloseManager<C>;

trait IdentifyGatewayConnection<E>: 
    Sink<GatewayOutgoing, Error = E> + 
    Stream<Item = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>> + 
    Unpin + 
    Sized
where 
    E: std::fmt:: Debug,
{
    async fn identify_gateway_connection(mut self, token: &str, intents: Intents) -> GatewayConnectionIdentified<Self>
    {

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
            intents: intents.bits(),
        };

        self.send(GatewayOutgoing::Send(GatewaySendEvent::Identify(identify))).await.unwrap();

        let sequence_number: Option<u64>;
        let resume_gateway_url: String;
        let session_id: String;

        match self.next().await {
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

        let close_manager = CloseManager::new(self, &resume_gateway_url, &token, &session_id, sequence_number.unwrap());

        return close_manager
    }
}

impl IdentifyGatewayConnection<GatewayError> for GatewayConnectionConnected {}

#[pin_project::pin_project()]
pub struct CloseManager<Inner> {
    #[pin]
    inner: Inner,
    inner_fut: Option<Pin<Box<dyn Future<Output = Option<Inner>>>>>,

    resume_gateway_url: String,
    token: String,
    session_id: String,
    sequence_number: u64,
}

impl<Inner> CloseManager<Inner> {
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

impl Stream for CloseManager<GatewayConnectionConnected>
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
                    }));
                    continue
                }
                Some(Err(e)) => return Poll::Ready(Some(Err(e)))
            }
        }
    }
}

impl Sink<GatewayOutgoing> for CloseManager<GatewayConnectionConnected> {
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
        let mut interval = interval(heartbeat_interval);
        interval.reset();

        HeartbeatManager { 
            sink,
            send_queue: VecDeque::new(),
            incoming, 
            interval, 
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



type TestHarness<St, Si, SiItem, Fut1, F1, Fut2, F2> = Duplex<futures::sink::With<Si, SiItem, SiItem, Fut1, F1>, futures::stream::Then<St, Fut2, F2>>;

struct TestHarnessBuilder;

impl TestHarnessBuilder {
    pub fn new<St, Si, StItem, SiItem, E, Fut1, F1, Fut2, F2>(
        f1: F1,
        f2: F2,
        sink: Si,
        stream: St,
    ) -> TestHarness<St, Si, SiItem, Fut1, F1, Fut2, F2>
    where
        St: Stream<Item = StItem>,
        Si: Sink<SiItem, Error = E>,
        Fut1: Future<Output = Result<SiItem, E>>,
        Fut2: Future<Output = StItem>,
        F1: FnMut(SiItem) -> Fut1,
        F2: FnMut(StItem) -> Fut2,
    {
        Duplex::new(sink.with(f1), stream.then(f2))
    }
}


impl<St, Si, Fut1, F1, Fut2, F2> CreateConnection for TestHarness<St, Si, GatewayOutgoing, Fut1, F1, Fut2, F2> 
where 
    St: Stream + 'static,
    Si: Sink<GatewayOutgoing, Error = GatewayError> + 'static,
    Fut1: Future<Output = Result<GatewayOutgoing, GatewayError>> + 'static,
    Fut2: Future<Output = Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>> + 'static,
    F1: FnMut(GatewayOutgoing) -> Fut1 + 'static,
    F2: FnMut(St::Item) -> Fut2 + 'static,

{}

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

    let gateway_url = format!("{}/?v=10", gateway_url);

    dotenvy::dotenv().unwrap();
    let token: &str = &std::env::var("GATEWAY_TOKEN").expect("Could not find token.");

    let (ws_sink, ws_stream) = connect_websocket(&gateway_url)
        .await
        .split();

    let print_outgoing = |msg: Message| async {
        match &msg {
            Message::Text(text) => {
                let raw: serde_json::Value = serde_json::from_str(&text).unwrap();
                println!("Sending JSON value: {:#?}", raw);
            },
            _ => {}
        }
        Ok(msg)
    };

    let print_incoming = |msg: Result<Message, tokio_tungstenite::tungstenite::Error>| async {
        match &msg {
            Ok(Message::Text(text)) => {
                let raw: serde_json::Value = serde_json::from_str(&text).unwrap();
                println!("Received JSON value: {:#?}", raw);
            },
            _ => {}
        }
        msg
    };

    let (ws_sink, ws_stream) = TestHarnessBuilder::new(
        print_outgoing, 
        print_incoming, 
        ws_sink, 
        ws_stream
    )
        .decode_connection()
        .split();

    let test_outgoing = |msg: GatewayOutgoing| async {
        info!("Sending message {:?}", msg);
        Ok(msg)
    };

    let (interval_tx, interval_rx) = tokio::sync::oneshot::channel();
    let mut interval_tx = Some(interval_tx);

    let test_incoming = move |msg: Result<GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent>, GatewayError>| {
        let interval_tx = interval_tx.take();
        async move {
            match &msg {
                Ok(GatewayIncoming::Recv(WithSequenceNumber{
                    inner: GatewayRecvEvent::Hello(hello),
                    ..
                })) => {
                    info!("Interval received.");
                    let _ = interval_tx.unwrap().send(Some(hello.heartbeat_interval));
                }
                _ => {
                    info!("Received message '{:?}'", msg);
                }
            }
            msg
        }
    };

    let test_harness = TestHarnessBuilder::new(
        test_outgoing, 
        test_incoming, 
        ws_sink, 
        ws_stream
    );

    let intents = Intents::GUILDS;

    let mut connection = test_harness 
        .create_connection()
        .await
        .identify_gateway_connection(token, intents)
        .await;

    println!("Connection established");

    println!("Waiting for heartbeat interval to arrive");

    let interval = interval_rx.await.unwrap().unwrap();

    println!("Interval arrived. Waiting {:?} ms", interval);

    // Spawn heartbeat loop
    let mut heartbeat_interval = tokio::time::interval(Duration::from_millis(interval));

    heartbeat_interval.reset();

    loop {
        tokio::select! {
            // Heartbeat tick
            _ = heartbeat_interval.tick() => {
                println!("Interval has passed. Closing connection.");

                connection.close().await.expect("Failed to close connection");

                println!("Connection closed");
            }

            // Incoming message
            msg = connection.next() => {
                match msg {
                    Some(Ok(_)) => {
                        match msg {
                            Some(Ok(GatewayRecvEvent::Dispatch(value))) => {
                                let guild: GuildCreate = serde_json::from_value(value).unwrap();
                                let channel = &guild.channels[2];
                                let channel_id = &channel.id;
                                println!("The channel id is: {:?}", channel_id);
                                println!("Sending test message");



                                // Discord API URL for sending messages
                                let url = format!("https://discord.com/api/v10/channels/{}/messages", channel_id);

                                #[derive(serde::Serialize, Debug)]
                                struct Message {
                                    content: String,
                                }

                                // Message to send
                                let message = Message {
                                    content: "Hello, world!".to_string(),
                                };

                                // Create the HTTP client
                                let client = reqwest::Client::new();

                                println!("Sending message: {:?}", message);

                                // Send the POST request
                                let response = client
                                    .post(url)
                                    .header("Authorization", format!("Bot {}", token))
                                    .json(&message)
                                    .send()
                                    .await;

                                match response {
                                    Ok(res) => {
                                        if res.status().is_success() {
                                            println!("Message sent successfully!");
                                        } else {
                                            // Print the response body to get more details on the error
                                            let status = res.status();
                                            let body = res.text().await.unwrap_or_else(|_| String::from("Failed to read response body"));
                                            println!("Failed to send message: {} - Response body: {}", status, body);
                                        }
                                    }
                                    Err(e) => println!("Error sending request: {}", e),
                                }
                            },
                            _ => {}
                        }
                    }
                    None => {
                        println!("Socket closed by server");
                        break;
                    }
                    Some(Err(e)) => {
                        println!("Socket closed with error");
                        break;
                    }
                }
            }
        }
    }

}




