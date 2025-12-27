use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Message, Utf8Bytes};

use crate::{close_codes::GatewayCloseCode, opcodes::GatewayOpCode, GatewayError, RawGatewayPayload};


impl From<GatewayRecvEvent> for Option<HeartbeatAck> {
    fn from(value: GatewayRecvEvent) -> Self {
        match value {
            GatewayRecvEvent::HeartbeatAck(ack) => Some(ack),
            _ => None,
        }
    }
}

impl TryFrom<Utf8Bytes> for GatewayRecvEvent {
    type Error = serde_json::Error;

    fn try_from(value: Utf8Bytes) -> Result<Self, Self::Error> {
        serde_json::from_str(value.as_str())
    }
}

impl From<Option<CloseFrame>> for GatewayCloseEvent {
    fn from(value: Option<CloseFrame>) -> Self {
        match value {
            Some(close_frame) => GatewayCloseEvent { close_code: Some(close_frame.code.into()), reason: close_frame.reason.to_string() },
            None => GatewayCloseEvent { close_code: None, reason: "".to_string() }
        }
    }
}

impl From<GatewayCloseEvent> for Message {
    fn from(value: GatewayCloseEvent) -> Self {
        Message::Close(value.into())
    }
}

impl From<GatewayCloseEvent> for Option<CloseFrame> {
    fn from(value: GatewayCloseEvent) -> Self {
        match value.close_code {
            Some(code) => Some(CloseFrame { code: code.into(), reason: value.reason.into() }),
            None => None
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum GatewayIncoming {
    Recv(GatewayRecvEvent),
    Close(GatewayCloseEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct GatewayCloseEvent {
    pub close_code: Option<GatewayCloseCode>,
    pub reason: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum GatewayRecvEvent {
    Hello(Hello),
    HeartbeatAck(HeartbeatAck),
    Heartbeat(Heartbeat),
    Ready(Ready),
    Reconnect(Reconnect),
    Resumed(Resumed),
    InvalidSession(InvalidSession),
}

impl TryFrom<RawGatewayPayload> for GatewayRecvEvent {
    type Error = GatewayError;

    fn try_from(raw: RawGatewayPayload) -> Result<Self, Self::Error> {
        
        let opcode = GatewayOpCode::try_from(raw.op)
            .map_err(|_e| GatewayError::InvalidOpCode(raw.op))?;

        match opcode {
            GatewayOpCode::Hello => {
                serde_json::from_value(raw.d)
                    .map(GatewayRecvEvent::Hello)
                    .map_err(|e| e.into())
            }
            GatewayOpCode::Heartbeat => {
                serde_json::from_value(raw.d)
                    .map(GatewayRecvEvent::Heartbeat)
                    .map_err(|e| e.into())
            }
            GatewayOpCode::HeartbeatAck => {
                Ok(GatewayRecvEvent::HeartbeatAck(HeartbeatAck))
            }
            GatewayOpCode::Reconnect => Ok(GatewayRecvEvent::Reconnect(Reconnect)),
            GatewayOpCode::InvalidSession => {
                serde_json::from_value(raw.d)
                    .map(GatewayRecvEvent::InvalidSession)
                    .map_err(|e| e.into())
            }
            _ => Err(GatewayError::InvalidOpCode(raw.op)),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Ready {
    pub v: u32,
    pub user: User,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<(u32, u32)>,
    pub application: Option<ApplicationInfo>,
    #[serde(default)]
    pub guilds: Vec<Guild>,
}

// Supporting types:

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    // Add other user fields as needed
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct ApplicationInfo {
    pub id: String,
    pub flags: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
    // Add other fields as needed
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub owner: Option<bool>,
    pub permissions: Option<String>,
    // Add other guild fields as needed
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct HeartbeatAck;
    
#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Reconnect;

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Resumed;

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct InvalidSession {
    pub resumable: bool,
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct Heartbeat {
    pub seq: Option<u64>,
}
