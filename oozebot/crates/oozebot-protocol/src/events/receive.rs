use serde::{Deserialize, Deserializer};
use serde_json::Value;

use crate::{opcodes::GatewayOpCode, GatewayError, RawGatewayPayload};


impl From<GatewayRecvEvent> for Option<HeartbeatAck> {
    fn from(value: GatewayRecvEvent) -> Self {
        match value {
            GatewayRecvEvent::HeartbeatAck(ack) => Some(ack),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum GatewayRecvEvent {
    Hello(Hello),
    HeartbeatAck(HeartbeatAck),
    Heartbeat(Heartbeat),
    Ready(Ready),
    Reconnect(Reconnect),
    Resumed(Resumed),
    InvalidSession(InvalidSession),
}

impl<'de> Deserialize<'de> for GatewayRecvEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawGatewayPayload::deserialize(deserializer)?;

        let opcode = GatewayOpCode::try_from(raw.op)
            .map_err(serde::de::Error::custom)?;

        match opcode {
            GatewayOpCode::Hello => {
                serde_json::from_value(raw.d)
                    .map(GatewayRecvEvent::Hello)
                    .map_err(serde::de::Error::custom)
            }
            GatewayOpCode::Heartbeat => {
                serde_json::from_value(raw.d)
                    .map(GatewayRecvEvent::Heartbeat)
                    .map_err(serde::de::Error::custom)
            }
            GatewayOpCode::HeartbeatAck => Ok(GatewayRecvEvent::HeartbeatAck(
                HeartbeatAck{
                    sequence_number: raw.s.ok_or(serde::de::Error::custom("sequence number not found in HeartbeatAck"))?
                }
            )),
            GatewayOpCode::Reconnect => Ok(GatewayRecvEvent::Reconnect(Reconnect)),
            GatewayOpCode::InvalidSession => {
                serde_json::from_value(raw.d)
                    .map(GatewayRecvEvent::InvalidSession)
                    .map_err(serde::de::Error::custom)
            }
            _ => Err(serde::de::Error::custom(GatewayError::InvalidOpCode(raw.op))),
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Dispatch {
    pub op: u8,            // Should be 0
    pub d: Value,          // Raw event payload (will depend on event type)
    pub s: Option<u64>,    // Sequence number
    pub t: Option<String>, // Event name, e.g. "MESSAGE_CREATE", "READY", etc.
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct HeartbeatAck {
    pub sequence_number: u64,
}
    
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
