use close_codes::GatewayCloseCode;
use events::dispatch::DispatchEvent;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use tokio_tungstenite::tungstenite;
use std::error::Error;
use std::fmt;


pub mod opcodes;
pub mod close_codes;
pub mod events;
pub mod intents;
pub mod resources;

type Seq = Option<u64>;

#[derive(Debug)]
pub struct WithSequenceNumber<T> {
    pub inner: T,
    pub sequence_number: Seq,
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
    SerdeError(#[from] BetterSerdeError),
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

pub struct BetterSerdeError {
    error: serde_json::Error,
    input: String,
}

impl BetterSerdeError {
    pub fn new(error: serde_json::Error, input: impl Into<String>) -> Self {
        Self {
            error,
            input: input.into(),
        }
    }


    fn render_highlight(&self) -> String {
        let line = self.error.line();
        let column = self.error.column();

        let mut output = String::new();
        let lines: Vec<&str> = self.input.lines().collect();

        if let Some(error_line) = lines.get(line.saturating_sub(1)) {
            output.push_str(&format!("{:>4} | {}\n", line, error_line));
            output.push_str(&format!(
                "     | {:>width$}\x1b[31m^\x1b[0m\n",
                "",
                width = column.saturating_sub(1),
            ));
        }

        output.push_str(&format!("\nError: {}\n", self.error));

        output
    }

}

impl fmt::Display for BetterSerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "JSON parse error: {}", self.error)?;
        writeln!(f, " --> line {}, column {}", self.error.line(), self.error.column())?;
        writeln!(f)?;
        write!(f, "{}", self.render_highlight())
    }
}

impl fmt::Debug for BetterSerdeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for BetterSerdeError {}

impl From<(serde_json::Error, &Value)> for BetterSerdeError {
    fn from((error, input): (serde_json::Error, &Value)) -> Self {
        BetterSerdeError::new(error, input.to_string())
    }
}

impl From<(serde_json::Error, &str)> for BetterSerdeError {
    fn from((error, input): (serde_json::Error, &str)) -> Self {
        BetterSerdeError::new(error, input.to_string())
    }
}

#[derive(Error, Debug)]
#[error("Heartbeat Timeout")]
pub struct HeartbeatError {}

#[derive(Debug, Deserialize)]
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


