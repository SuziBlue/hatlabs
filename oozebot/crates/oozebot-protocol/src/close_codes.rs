
use serde::{Deserialize, Deserializer, Serialize};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;


/// Discord Gateway Close Codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[repr(u16)]
pub enum GatewayCloseCode {
    /// Unknown error — We're not sure what went wrong. Try reconnecting? (Reconnect: true)
    UnknownError = 4000,
    /// Unknown opcode — You sent an invalid Gateway opcode or payload. (Reconnect: true)
    UnknownOpcode = 4001,
    /// Decode error — You sent an invalid payload to Discord. (Reconnect: true)
    DecodeError = 4002,
    /// Not authenticated — You sent a payload before identifying, or session invalidated. (Reconnect: true)
    NotAuthenticated = 4003,
    /// Authentication failed — The token sent was incorrect. (Reconnect: false)
    AuthenticationFailed = 4004,
    /// Already authenticated — You sent more than one identify payload. (Reconnect: true)
    AlreadyAuthenticated = 4005,
    /// Invalid seq — Sequence sent when resuming was invalid. Reconnect and start a new session. (Reconnect: true)
    InvalidSeq = 4007,
    /// Rate limited — You're sending payloads too quickly. (Reconnect: true)
    RateLimited = 4008,
    /// Session timed out — Your session timed out. Reconnect and start a new one. (Reconnect: true)
    SessionTimedOut = 4009,
    /// Invalid shard — You sent an invalid shard when identifying. (Reconnect: false)
    InvalidShard = 4010,
    /// Sharding required — Too many guilds. You must shard to connect. (Reconnect: false)
    ShardingRequired = 4011,
    /// Invalid API version — You sent an invalid gateway version. (Reconnect: false)
    InvalidApiVersion = 4012,
    /// Invalid intent(s) — You sent an invalid intent value. (Reconnect: false)
    InvalidIntents = 4013,
    /// Disallowed intent(s) — You sent an intent you’re not enabled or approved for. (Reconnect: false)
    DisallowedIntents = 4014,
    /// An unknown or undocumented close code.
    Unknown(u16),
}

impl GatewayCloseCode {
    /// Returns true if the client is allowed or recommended to reconnect.
    pub fn can_reconnect(&self) -> bool {
        matches!(
            self,
            GatewayCloseCode::UnknownError
                | GatewayCloseCode::UnknownOpcode
                | GatewayCloseCode::DecodeError
                | GatewayCloseCode::NotAuthenticated
                | GatewayCloseCode::AlreadyAuthenticated
                | GatewayCloseCode::InvalidSeq
                | GatewayCloseCode::RateLimited
                | GatewayCloseCode::SessionTimedOut
        )
    }
}

impl From<u16> for GatewayCloseCode {
    fn from(value: u16) -> Self {
        match value {
            4000 => GatewayCloseCode::UnknownError,
            4001 => GatewayCloseCode::UnknownOpcode,
            4002 => GatewayCloseCode::DecodeError,
            4003 => GatewayCloseCode::NotAuthenticated,
            4004 => GatewayCloseCode::AuthenticationFailed,
            4005 => GatewayCloseCode::AlreadyAuthenticated,
            4007 => GatewayCloseCode::InvalidSeq,
            4008 => GatewayCloseCode::RateLimited,
            4009 => GatewayCloseCode::SessionTimedOut,
            4010 => GatewayCloseCode::InvalidShard,
            4011 => GatewayCloseCode::ShardingRequired,
            4012 => GatewayCloseCode::InvalidApiVersion,
            4013 => GatewayCloseCode::InvalidIntents,
            4014 => GatewayCloseCode::DisallowedIntents,
            other => GatewayCloseCode::Unknown(other),
        }
    }
}

impl TryFrom<CloseCode> for GatewayCloseCode {
    type Error = ();

    fn try_from(value: CloseCode) -> Result<Self, Self::Error> {
        match value {
            CloseCode::Library(code) => Ok(code.into()),
            _ => Err(()),
        }
    }
}

impl<'de> Deserialize<'de> for GatewayCloseCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;
        Ok(GatewayCloseCode::from(code))
    }
}


