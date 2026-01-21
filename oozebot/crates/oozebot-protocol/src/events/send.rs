use serde::Serialize;
use tokio_tungstenite::tungstenite;

use crate::{close_codes::GatewayCloseEvent, opcodes::GatewayOpCode};


impl From<Heartbeat> for GatewaySendEvent {
    fn from(value: Heartbeat) -> Self {
        GatewaySendEvent::Heartbeat(value)
    }
}

impl From<GatewaySendEvent> for tungstenite::Message {
    fn from(value: GatewaySendEvent) -> Self {
        let payload: GatewayPayload<GatewaySendEvent> = value.into();
        let text = serde_json::to_string(&payload).expect("Should be serializable");
        tungstenite::Message::text(text)
    }
}

impl From<GatewayOutgoing> for tungstenite::Message {
    fn from(value: GatewayOutgoing) -> Self {
        match value {
            GatewayOutgoing::Send(send) => send.into(),
            GatewayOutgoing::Close(close) => close.into(),
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct GatewayPayload<T> {
    pub op: GatewayOpCode,
    pub d: T
}

impl From<GatewaySendEvent> for GatewayPayload<GatewaySendEvent> {
    fn from(value: GatewaySendEvent) -> Self {
        let op = match value {
            GatewaySendEvent::Identify(_) => GatewayOpCode::Identify,
            GatewaySendEvent::Resume(_) => GatewayOpCode::Resume,
            GatewaySendEvent::Heartbeat(_) => GatewayOpCode::Heartbeat,
            GatewaySendEvent::RequestGuildMembers(_) => GatewayOpCode::RequestGuildMembers,
            GatewaySendEvent::RequestSoundboardSounds(_) => GatewayOpCode::RequestSoundboardSounds,
            GatewaySendEvent::UpdatePresence(_) => GatewayOpCode::PresenceUpdate,
            GatewaySendEvent::UpdateVoiceState(_) => GatewayOpCode::VoiceStateUpdate,
        };

        GatewayPayload { 
            op, 
            d: value 
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum GatewaySendEvent {
    Identify(Identify),
    Resume(Resume),
    Heartbeat(Heartbeat),
    RequestGuildMembers(RequestGuildMembers),
    RequestSoundboardSounds(RequestSoundboardSounds),
    UpdateVoiceState(UpdateVoiceState),
    UpdatePresence(UpdatePresence),
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub enum GatewayOutgoing {
    Send(GatewaySendEvent),
    Close(GatewayCloseEvent),
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Identify {
    pub token: String,
    pub properties: ClientProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<(u64, u64)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
    pub intents: u64,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct ClientProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Presence {
    pub status: String,
    pub activities: Vec<Activity>,
    pub afk: bool,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Activity {
    pub name: String,
    pub kind: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Party>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Assets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Secrets>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<u64>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Emoji {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<(u64, u64)>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Assets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Secrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Heartbeat(pub Option<u64>);

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RequestGuildMembers {
    pub guild_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RequestSoundboardSounds {
    pub guild_ids: Vec<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct UpdateVoiceState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct UpdatePresence {
    pub status: String,
    pub afk: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct ClientStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}
