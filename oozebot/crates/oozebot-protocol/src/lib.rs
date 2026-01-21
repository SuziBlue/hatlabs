use close_codes::GatewayCloseCode;
use events::dispatch::DispatchEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

pub mod opcodes;
pub mod close_codes;
pub mod events;
pub mod intents;

type Seq = Option<u64>;

pub struct WithSequenceNumber<T> {
    inner: T,
    sequence_number: Seq,
}

impl<T> WithSequenceNumber<T> {
    pub fn wrap(inner: T, sequence_number: Seq) -> Self {
        Self { inner, sequence_number }
    }
    pub fn into_inner(self) -> T {
        self.inner
    }
    pub fn inner_ref(&self) -> &T {
        &self.inner
    }
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    pub fn sequence_number(&self) -> Seq {
        self.sequence_number
    }
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> WithSequenceNumber<U> {
        WithSequenceNumber { inner: f(self.inner), sequence_number: self.sequence_number }
    }
}

#[derive(Error, Debug)]
pub enum GatewayError {
    #[error("Invalid op code: {0}")]
    InvalidOpCode(u8),
    #[error("Invalid json: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Websocket error: {0}")]
    WebSocketError(#[from] tungstenite::Error),
    #[error("Heartbeat error: {0}")]
    HeartbeatError(#[from] HeartbeatError),
    #[error("Connection closed by server with code: {:?}", 0)]
    Closed(GatewayCloseCode),
    #[error("Failed to resume connection")]
    ResumeError,
    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

#[derive(Error, Debug)]
#[error("Heartbeat Timeout")]
pub struct HeartbeatError {}

#[derive(Deserialize)]
pub struct RawGatewayPayload {
    op: u8,
    #[serde(default)]
    d: Value,
    pub s: Option<u64>,
    pub t: Option<DispatchEvent>,
}

#[cfg(test)]
mod tests {
    use crate::events::receive::GatewayRecvEvent;

    #[test]
    fn deserialize_hello_event() {
        let json_data = r#"
        {
            "op": 10,
            "d": {
                "heartbeat_interval": 41250
            }
        }
        "#;

        let event: GatewayRecvEvent =
            serde_json::from_str(json_data).expect("Failed to deserialize");

        match event {
            GatewayRecvEvent::Hello(e) => {
                assert_eq!(e.heartbeat_interval, 41250);
            }
            _ => {panic!("Incorrect event variant {:?}", event)}
        }
    }
}


