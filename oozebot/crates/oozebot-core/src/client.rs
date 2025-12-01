use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;

use anyhow::{Ok, Result};
use anyhow::anyhow;
use thiserror::Error;
use serde_json::Value;

use crate::connection::Connection;


const DISCORD_API_URL: &str = "https://discord.com/api/v10";

pub struct DiscordClient {
    connection: Connection,
    gateway_url: Option<String>,
    resume_gateway_url: Option<String>,
}

impl DiscordClient {
    pub async fn new() -> Result<Self> {
    
        let connection = Connection::new();

        let gateway_url = Self::get_gateway_url(DISCORD_API_URL).await?;



        Ok(Self { 
            connection, 
            gateway_url: Some(gateway_url.to_string()), 
            resume_gateway_url: None 
        })

    }

    async fn get_gateway_url(api_url: &str) -> Result<String> {
        let body = reqwest::get(format!("{}/gateway", api_url))
            .await?
            .text()
            .await?;

        let value: Value = serde_json::from_str(&body)?;

        let gateway_url = value["url"].to_string();

        Ok(gateway_url)
    }

    pub async fn connect(&mut self) -> Result<()> {

        let gateway_url = self.gateway_url.as_ref().ok_or(anyhow!("No gateway url found."))?;

        self.connection.connect_to_websocket(gateway_url);


        Ok(())



    }

    pub async fn resume(self) -> Result<DiscordClient> {

        let resume_gateway_url = self
            .connection_info
            .resume_gateway_url
            .ok_or(anyhow!("Tried to resume gateway connection, but no resume gateway url found."))?;

        let (stream, _response) = connect_async(resume_gateway_url.clone()).await?;

        let (write, read) = stream.split();

        let event_handler = self.event_handler.expect("Event handler should exist.");

        let event_sender = self.event_sender.expect("Event sender should exist.");

        event_handler.update_stream(read);

        event_sender.update_stream(write);



        let resume_payload = GatewaySendEvent::Resume(
            Resume { 
                token: token, 
                session_id: session_id, 
                seq: sequence_number 
            }
        );

        let _ = event_handler.wait_for_event(|event| {
            if let GatewayRecvEvent::Resumed(resumed) = event {
                Some(resumed)
            } else {
                None
            }
        }).await?;

        let connection_info = ConnectionInfo {
            connection_state: ConnectionState::Identified,
            gateway_url: self.connection_info.gateway_url,
            resume_gateway_url: Some(resume_gateway_url),
        };

        Ok(DiscordClient {
            connection_info,
            event_handler: Some(event_handler),
            event_sender: Some(event_sender),
        })

    }

    pub async fn identify(self, token: String, intents: Intents) -> Result<DiscordClient> {

        let identify_payload = GatewaySendEvent::Identify(
            Identify { 
                token, 
                properties: ClientProperties {
                    os: std::env::consts::OS.to_string(),
                    browser: "slimebot".to_string(),
                    device: "slimebot".to_string(),
                }, 
                compress: None, 
                large_threshold: None, 
                shard: None, 
                presence: None, 
                intents: intents.bits(), 
            }
        );

        let event_handler = self.event_handler.expect("Event handler should exist.");
        let event_sender = self.event_sender.expect("Event sender should exist.");

        let ready_fut = event_handler.wait_for_event(|event| {
            if let GatewayRecvEvent::Ready(ready) = event {
                Some(ready)
            } else {
                None
            }
        });

        event_sender.event_tx.send(identify_payload).await?;

        let ready = ready_fut.await?;

        let resume_gateway_url = ready.resume_gateway_url.clone();

        let connection_info = ConnectionInfo { 
            connection_state: ConnectionState::Identified, 
            gateway_url: self.connection_info.gateway_url, 
            resume_gateway_url: Some(resume_gateway_url), 
        };

        Ok(DiscordClient { 
            connection_info,
            event_handler: Some(event_handler), 
            event_sender: Some(event_sender), 
        })
    }
}



mod tests {
    use crate::client::DiscordClient;



    #[tokio::test]
    async fn test_gateway() {
        let client = DiscordClient::new();
        let result = client.connect().await;
        assert!(!result.is_err())
    }
}
