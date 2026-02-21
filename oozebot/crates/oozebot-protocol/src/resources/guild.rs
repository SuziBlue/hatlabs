use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde::de::Error;

pub type Snowflake = String;

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
pub struct UnavailableGuild {
    pub id: String,
    pub unavailable: bool,
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Guild {
    pub id: Snowflake,

    pub name: String,

    #[serde(default)]
    pub icon: Option<String>,

    #[serde(default)]
    pub splash: Option<String>,

    #[serde(default)]
    pub discovery_splash: Option<String>,

    pub owner_id: Snowflake,

    pub afk_timeout: u32,

    pub verification_level: u8,
    pub default_message_notifications: u8,
    pub explicit_content_filter: u8,
    pub mfa_level: u8,
    pub nsfw_level: u8,

    pub premium_tier: u8,
    pub premium_subscription_count: Option<u32>,
    pub premium_progress_bar_enabled: bool,

    pub preferred_locale: String,

    pub system_channel_flags: u32,

    pub system_channel_id: Option<Snowflake>,

    pub rules_channel_id: Option<Snowflake>,

    pub public_updates_channel_id: Option<Snowflake>,

    pub safety_alerts_channel_id: Option<Snowflake>,

    pub max_members: Option<u32>,
    pub max_video_channel_users: Option<u32>,
    pub max_stage_video_channel_users: Option<u32>,

    pub description: Option<String>,
    pub banner: Option<String>,
    pub vanity_url_code: Option<String>,

    #[serde(default)]
    pub features: Vec<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GuildCreate {
    #[serde(flatten)]
    pub guild: Guild,

    // Gateway-only fields
    pub joined_at: String,
    pub large: bool,
    pub unavailable: Option<bool>,
    pub member_count: u32,

    #[serde(default)]
    pub voice_states: Vec<VoiceState>,

    #[serde(default)]
    pub members: Vec<Member>,

    #[serde(default)]
    pub channels: Vec<Channel>,

    #[serde(default)]
    pub threads: Vec<Channel>,

    #[serde(default)]
    pub presences: Vec<Presence>,

    #[serde(default)]
    pub stage_instances: Vec<StageInstance>,

    #[serde(default)]
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,

    #[serde(default)]
    pub soundboard_sounds: Vec<SoundboardSound>,
}


#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct Channel {
    pub id: Snowflake,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "type")]
    pub kind: u8,
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct VoiceState {
    #[serde(default)]
    pub channel_id: Option<String>,

    #[serde(default)]
    pub mute: Option<bool>,

    #[serde(default)]
    pub deaf: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct Member {
    pub user: User,

    #[serde(default)]
    pub nick: Option<String>,

    #[serde(default)]
    pub roles: Vec<String>,

    pub joined_at: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct User {
    pub id: Snowflake,

    pub username: String,
    pub discriminator: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Presence {
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct StageInstance {
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GuildScheduledEvent {
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SoundboardSound {
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

