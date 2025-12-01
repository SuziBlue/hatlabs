use serde::{Serialize, Deserialize};
use crate::GatewayError;


/// Gateway operation codes used in the Discord Gateway API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[serde(try_from = "u8")]
pub enum GatewayOpCode {
    /// 0 - An event was dispatched.
    Dispatch = 0,

    /// 1 - Fired periodically by the client to keep the connection alive.
    Heartbeat = 1,

    /// 2 - Starts a new session during the initial handshake.
    Identify = 2,

    /// 3 - Update the client's presence.
    PresenceUpdate = 3,

    /// 4 - Used to join/leave or move between voice channels.
    VoiceStateUpdate = 4,

    /// 6 - Resume a previous session that was disconnected.
    Resume = 6,

    /// 7 - You should attempt to reconnect and resume immediately.
    Reconnect = 7,

    /// 8 - Request information about offline guild members in a large guild.
    RequestGuildMembers = 8,

    /// 9 - The session has been invalidated. You should reconnect and identify/resume accordingly.
    InvalidSession = 9,

    /// 10 - Sent immediately after connecting, contains the heartbeat_interval to use.
    Hello = 10,

    /// 11 - Sent in response to receiving a heartbeat to acknowledge that it has been received.
    HeartbeatAck = 11,

    /// 31 - Request information about soundboard sounds in a set of guilds.
    RequestSoundboardSounds = 31,
}


impl TryFrom<u8> for GatewayOpCode {
    type Error = GatewayError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Dispatch),
            1 => Ok(Self::Heartbeat),
            2 => Ok(Self::Identify),
            3 => Ok(Self::PresenceUpdate),
            4 => Ok(Self::VoiceStateUpdate),
            6 => Ok(Self::Resume),
            7 => Ok(Self::Reconnect),
            8 => Ok(Self::RequestGuildMembers),
            9 => Ok(Self::InvalidSession),
            10 => Ok(Self::Hello),
            11 => Ok(Self::HeartbeatAck),
            31 => Ok(Self::RequestSoundboardSounds),
            _ => Err(Self::Error::InvalidOpCode(value)),
        }
    }
}
