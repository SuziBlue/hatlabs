use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio_tungstenite::tungstenite;

pub mod opcodes;
pub mod close_codes;
pub mod events;
pub mod intents;



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
}

#[derive(Error, Debug)]
#[error("Heartbeat Timeout")]
pub struct HeartbeatError {}

#[derive(Serialize, Deserialize)]
pub struct RawGatewayPayload {
    op: u8,
    #[serde(default)]
    d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
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


