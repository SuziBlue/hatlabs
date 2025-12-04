use serde::Serialize;
use tokio_tungstenite::tungstenite::{self, Utf8Bytes};

impl From<Heartbeat> for GatewaySendEvent {
    fn from(value: Heartbeat) -> Self {
        GatewaySendEvent::Heartbeat(value)
    }
}

impl From<GatewaySendEvent> for tungstenite::Message {
    fn from(value: GatewaySendEvent) -> Self {
        let text = serde_json::to_string(&value).expect("Should be serializable");
        tungstenite::Message::text(text)
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
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
pub struct Identify {
    pub token: String,
    pub properties: ClientProperties,
    pub compress: Option<bool>,
    pub large_threshold: Option<u64>,
    pub shard: Option<(u64, u64)>,
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
    pub url: Option<String>,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<Assets>,
    pub secrets: Option<Secrets>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Emoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Party {
    pub id: Option<String>,
    pub size: Option<(u64, u64)>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Assets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Secrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    pub match_: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Resume {
    pub token: String,
    pub session_id: String,
    pub seq: u64,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct Heartbeat {
    pub d: Option<u64>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RequestGuildMembers {
    pub guild_id: String,
    pub query: Option<String>,
    pub limit: Option<u64>,
    pub presences: Option<bool>,
    pub user_ids: Option<Vec<String>>,
    pub nonce: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct RequestSoundboardSounds {
    pub guild_ids: Vec<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct UpdateVoiceState {
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub self_mute: bool,
    pub self_deaf: bool,
    pub suppress: Option<bool>,
    pub request_to_speak_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct UpdatePresence {
    pub status: String,
    pub afk: bool,
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

#[derive(Debug, Serialize, Clone, PartialEq, PartialOrd)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
}
