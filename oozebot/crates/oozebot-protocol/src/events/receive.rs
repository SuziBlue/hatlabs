use serde::Deserialize;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Utf8Bytes;

use crate::{close_codes::GatewayCloseEvent, opcodes::GatewayOpCode, resources::guild::{GuildCreate, UnavailableGuild}, BetterSerdeError, GatewayError, RawGatewayPayload, WithSequenceNumber};

use super::dispatch::DispatchEvent;


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


#[derive(Debug)]
pub enum GatewayIncoming<Recv, Close> {
    Recv(Recv),
    Close(Close),
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum GatewayRecvEvent {
    Hello(Hello),
    HeartbeatAck(HeartbeatAck),
    Heartbeat(Heartbeat),
    Ready(Ready),
    Reconnect(Reconnect),
    Resumed(Resumed),
    InvalidSession(InvalidSession),
    Dispatch(Value),
}

impl TryFrom<RawGatewayPayload> for GatewayIncoming<WithSequenceNumber<GatewayRecvEvent>, GatewayCloseEvent> {
    type Error = GatewayError;

    fn try_from(raw: RawGatewayPayload) -> Result<Self, Self::Error> {
        
        let opcode = GatewayOpCode::try_from(raw.op)
            .map_err(|_e| GatewayError::InvalidOpCode(raw.op))?;

        let sequence_number = raw.s;

        match opcode {
            GatewayOpCode::Hello => {
                serde_json::from_value(raw.d.clone())
                    .map(GatewayRecvEvent::Hello)
                    .map_err(|e| Into::<BetterSerdeError>::into((e,&raw.d)).into())
            }
            GatewayOpCode::Heartbeat => {
                serde_json::from_value(raw.d.clone())
                    .map(GatewayRecvEvent::Heartbeat)
                    .map_err(|e| Into::<BetterSerdeError>::into((e,&raw.d)).into())
            }
            GatewayOpCode::HeartbeatAck => {
                Ok(GatewayRecvEvent::HeartbeatAck(HeartbeatAck))
            }
            GatewayOpCode::Reconnect => {
                Ok(GatewayRecvEvent::Reconnect(Reconnect))
            },
            GatewayOpCode::InvalidSession => {
                serde_json::from_value(raw.d.clone())
                    .map(GatewayRecvEvent::InvalidSession)
                    .map_err(|e| Into::<BetterSerdeError>::into((e,&raw.d)).into())
            },
            GatewayOpCode::Dispatch => {
                let event_name = raw.t
                    .ok_or(GatewayError::ProtocolError("Dispatch event received with no event name.".to_string()))?;

                match event_name {
                    DispatchEvent::Ready => {
                        serde_json::from_value(raw.d.clone())
                            .map(GatewayRecvEvent::Ready)
                            .map_err(|e| Into::<BetterSerdeError>::into((e, &raw.d)).into())
                    },
                    _ => {
                        serde_json::from_value(raw.d.clone())
                            .map(GatewayRecvEvent::Dispatch)
                            .map_err(|e| Into::<BetterSerdeError>::into((e, &raw.d)).into())
                    },
                }


            }
            _ => panic!("Opcode not implemented yet: {:?}", opcode),

        }
        .map(|recv| WithSequenceNumber::wrap(recv, sequence_number))
        .map(GatewayIncoming::Recv)
    }
}

impl<Recv> From<GatewayCloseEvent> for GatewayIncoming<Recv, GatewayCloseEvent> {
    fn from(value: GatewayCloseEvent) -> Self {
        GatewayIncoming::Close(value)
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
    pub guilds: Vec<UnavailableGuild>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum GuildCreateEvent {
    UnavailableGuild(UnavailableGuild),
    GuildCreate(GuildCreate),
}

impl<'de> Deserialize<'de> for GuildCreateEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if value.get("unavailable")
            .and_then(|v| v.as_bool())
            == Some(true)
        {
            Ok(GuildCreateEvent::UnavailableGuild(
                serde_json::from_value(value)
                    .map_err(serde::de::Error::custom)?,
            ))
        } else {
            Ok(GuildCreateEvent::GuildCreate(
                serde_json::from_value(value)
                    .map_err(serde::de::Error::custom)?,
            ))
        }
    }
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
