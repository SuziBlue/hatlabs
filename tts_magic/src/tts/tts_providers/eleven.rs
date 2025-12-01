
use std::collections::VecDeque;

use base64::engine::general_purpose;
use base64::Engine;
use futures::StreamExt;
use futures::SinkExt;
use log::info;
use anyhow::{anyhow, Result};
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use http::{HeaderName, HeaderValue};
use serde_json::json;
use crossbeam_channel::{unbounded, bounded, Sender, Receiver};
use serde::{Deserialize, Serialize};


use crate::tts::{AlignmentChunk, TtsModule, TtsOutput};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ElevenLabsTtsMessage {
    AudioChunk {
        #[serde(rename = "audio")]
        audio_base64: String,

        #[serde(rename = "alignment")]
        alignment: Option<AlignmentChunk>,

        #[serde(rename = "normalizedAlignment")]
        normalized_alignment: Option<AlignmentChunk>,

        #[serde(rename = "isFinal")]
        is_final: Option<bool>, // Optional and defaults to false
    },
    FinalMessage {
        #[serde(rename = "audio")]
        audio_base64: Option<serde_json::Value>, // could be null or missing

        #[serde(rename = "isFinal")]
        is_final: bool,

        #[serde(rename = "alignment")]
        alignment: Option<AlignmentChunk>,

        #[serde(rename = "normalizedAlignment")]
        normalized_alignment: Option<AlignmentChunk>,
    },
}

//impl<'de> Deserialize<'de> for AlignmentChunk {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//    where
//        D: Deserializer<'de>,
//    {
//        #[derive(Deserialize)]
//        struct AlignmentChunkRaw {
//            #[serde(rename = "chars")]
//            chars: Vec<char>,
//            #[serde(rename = "charStartTimesMs")]
//            start_times: Vec<u64>,
//            #[serde(rename = "charDurationsMs")]
//            durations: Vec<u64>,
//        }
//
//        let raw = AlignmentChunkRaw::deserialize(deserializer)?;
//
//        let len = raw.chars.len();
//        if raw.start_times.len() != len || raw.durations.len() != len {
//            return Err(de::Error::custom("Mismatched lengths in alignment fields"));
//        }
//
//        let items = raw
//            .chars
//            .into_iter()
//            .zip(raw.start_times)
//            .zip(raw.durations)
//            .map(|((ch, start), dur)| AlignedChar {
//                ch,
//                start_ms: start,
//                duration_ms: dur,
//            })
//            .collect();
//
//        Ok(AlignmentChunk { items })
//    }
//}

pub struct ElevenTtsModule {
    input_tx: Sender<String>,
    output_rx: Receiver<VecDeque<f32>>,
    sender_handle: JoinHandle<Result<()>>,
    receiver_handle: JoinHandle<Result<TtsOutput>>,
}
impl ElevenTtsModule {
    pub async fn try_new(voice_id: String, api_key: String, model_id: String) -> Result<ElevenTtsModule> {

        let ws_url = format!(
            "wss://api.elevenlabs.io/v1/text-to-speech/{voice_id}/stream-input\
        ?output_format=pcm_24000\
        &model_id={model_id}\
        &language_code=en\
        &sync_alignment=true",
            voice_id = voice_id,
            model_id = model_id,
        );
        let mut request = ws_url.into_client_request()?;
        let api_key_header = HeaderName::from_static("xi-api-key");
        request.headers_mut().insert(api_key_header, HeaderValue::from_str(&api_key)?);

        let (ws_stream, _) = connect_async(request).await?;
        let (mut write, mut read) = ws_stream.split();

        let (input_tx, sender_rx): (Sender<String>, Receiver<String>) = unbounded();
        let (receiver_tx, output_rx): (Sender<VecDeque<f32>>, Receiver<VecDeque<f32>>) = unbounded();

        // Task: send text chunks
        let sender: JoinHandle<Result<()>> = tokio::spawn(async move {
            
            // Required initial message: "text": " "
            let init_msg = json!({
                "text": " ", // required blank space
                "try_trigger_generation": false,
                "voice_settings": {
                    "speed": 1.05,
                }
            });

            write.send(Message::Text(init_msg.to_string().into())).await?;
            info!("TTS Sender: ➡️ Sent initialization message.");

            // Step 2: Send chunks of text
            while let Ok(chunk) = sender_rx.recv() {
                if chunk == "" {
                    continue
                }

                let msg = json!({
                    "text": chunk,
                    "try_trigger_generation": true
                });

                write.send(Message::Text(msg.to_string().into())).await?;

                info!("TTS Sender: 📝 Sent chunk: {:?}", chunk);
            }

            // Step 3: Close the stream by sending empty string
            let close_msg = json!({
                "text": ""
            });

            write.send(Message::Text(close_msg.to_string().into())).await?;
            info!("TTS Sender: 👋 Sent close message.");

            Ok(())
        });

        // Task: receive audio chunks
        let receiver = tokio::spawn(async move {
            let mut full_audio = Vec::new();
            let mut full_alignment = AlignmentChunk::new();
            
            info!("TTS Recevier: Waiting for audio chunks.");

            while let Some(msg) = read.next().await {
                match msg? {
                    Message::Text(text) => {
                        info!("TTS Receiver: Received JSON chunk {}", text);

                        match serde_json::from_str::<ElevenLabsTtsMessage>(&text)? {
                            ElevenLabsTtsMessage::AudioChunk { audio_base64, alignment, .. } => {
                                let samples_f32: Vec<f32> = general_purpose::STANDARD
                                    .decode(&audio_base64)?
                                    .chunks_exact(2)
                                    .map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / i16::MAX as f32)
                                    .collect();

                                // (Optional) Create a richer type with audio + alignment
                                info!("TTS Receiver: Sending audio samples");
                                receiver_tx.send(samples_f32.clone().into())?;

                                // (Optional) Log or store alignment info
                                if let Some(align) = alignment {
                                    full_audio.extend(samples_f32);
                                    full_alignment.extend_from(align);
                                } else {
                                    return Err(anyhow!("TTS message did not contain alignment information."))
                                };
                            }
                            ElevenLabsTtsMessage::FinalMessage { .. } => {
                                info!("TTS Receiver: Received final message");

                                let tts_result = TtsOutput { audio: full_audio, alignment: full_alignment };
                                return Ok(tts_result)
                            }
                        }
                    },
                    _ => {
                        info!("TTS Receiver: Unknown message received.", );
                    }
                }
            }
            info!("TTS Receiver: Reader closed.");
            Err(anyhow!("TTS receiver closed without finishing."))
        });




        Ok(ElevenTtsModule { 
            sender_handle: sender, 
            receiver_handle: receiver,
            input_tx,
            output_rx 
        })
    }
}

impl TtsModule for ElevenTtsModule {

    fn connect_input(&self) -> Sender<String> {
        self.input_tx.clone()
    }

    fn connect_output(&self) -> Receiver<VecDeque<f32>> {
        self.output_rx.clone()
    }
    
    async fn join(self) -> Result<TtsOutput> {
        drop(self.input_tx);
        self.sender_handle.await??;
        let tts_result = self.receiver_handle.await?;
        drop(self.output_rx);
        tts_result
    }
}
