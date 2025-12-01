use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub mod opcodes;
pub mod close_codes;
pub mod events;
pub mod intents;



#[derive(Error, Debug, Clone)]
pub enum GatewayError {
    #[error("Invalid op code: {}", .0)]
    InvalidOpCode(u8),
}

#[derive(Serialize, Deserialize)]
struct RawGatewayPayload {
    op: u8,
    #[serde(default)]
    d: Value,
    s: Option<u64>,
    t: Option<String>,
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


