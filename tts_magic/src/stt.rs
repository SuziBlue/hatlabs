use std::time::Duration;
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::{Message, client::IntoClientRequest}};
use tokio::{sync::mpsc::{Receiver, Sender}, time::sleep};
use serde_json::json;
use log::{error, info};

pub async fn start(b64_stream: tokio::sync::broadcast::Sender<String>) -> anyhow::Result<tokio::sync::mpsc::Receiver<String>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;

    // Output transcript channel
    let (tx, rx): (Sender<String>, Receiver<String>) = tokio::sync::mpsc::channel(10);

    // Cloneable sender to pass into each WebSocket lifecycle
    let tx = std::sync::Arc::new(tokio::sync::Mutex::new(tx));

    tokio::spawn(async move {
        loop {
            let url = "wss://api.openai.com/v1/realtime?intent=transcription";

            let mut request = match url.into_client_request() {
                Ok(req) => req,
                Err(e) => {
                    error!("Failed to create request: {}", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            use reqwest::header::AUTHORIZATION;
            use oauth2::http::HeaderName;

            request.headers_mut().insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
            let beta_header = HeaderName::from_static("openai-beta");
            request.headers_mut().insert(beta_header, "realtime=v1".parse().unwrap());

            let ws_stream = match connect_async(request).await {
                Ok((stream, _)) => stream,
                Err(e) => {
                    error!("Failed to connect: {}", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };

            let (mut write, mut read) = ws_stream.split();

            // Send config
            let config_msg = json!({
                "type": "transcription_session.update",
                "session": {
                    "input_audio_format": "pcm16",
                    "input_audio_transcription": {
                        "model": "gpt-4o-transcribe",
                        "prompt": "You are transcribing audio from a talking hat to a slime girl called Suziblue or Suzi.",
                        "language": "en"
                    },
                    "turn_detection": {
                        "type": "semantic_vad",
                        "eagerness": "low",
                        //"idle_timeout_ms": 4000,
                    }
                }
            });

            if let Err(e) = write.send(Message::Text(config_msg.to_string().into())).await {
                error!("Error sending config: {}", e);
                continue;
            }

            let tx_clone = tx.clone();
            let mut b64_rx = b64_stream.subscribe();

            // Audio sender
            let sender_handle = tokio::spawn(async move {
                while let Ok(b64_audio) = b64_rx.recv().await {
                    let msg = json!({
                        "type": "input_audio_buffer.append",
                        "audio": b64_audio
                    });

                    if let Err(e) = write.send(Message::Text(msg.to_string().into())).await {
                        error!("WebSocket send error: {}", e);
                        break;
                    }
                }
            });

            let receiver_handle = tokio::spawn(async move {
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            // Print full raw server response
                            info!("Server Response: {}", text);
            
                            // Optional: Try parsing as JSON and handle specific types
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                                if json["type"] == "conversation.item.input_audio_transcription.completed" {
                                    let transcript = json["transcript"].to_string();
                                    if let Err(e) = tx_clone.lock().await.send(transcript).await {
                                        error!("Error sending transcript: {}", e);
                                    }
                                }
                            }
                        }
                        Ok(other) => {
                            info!("Non-text message received: {:?}", other);
                        }
                        Err(e) => {
                            error!("WebSocket read error: {}", e);
                            break;
                        }
                    }
                }
            });

            // Wait for 30 minutes or any of the tasks to finish
            tokio::select! {
                _ = sender_handle => {
                    error!("❌ Sender task exited unexpectedly, restarting...");
                }
                _ = receiver_handle => {
                    error!("❌ Receiver task exited unexpectedly, restarting...");
                }
            }

            // Tasks will be dropped and a new connection will be created
        }
    });

    Ok(rx)
}
