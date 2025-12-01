use derive_builder::Builder;
use log::error;
use log::info;
use reqwest::Client;
use reqwest::Error;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TTSRequest {
    pub text: String,
    #[serde(flatten)]
    pub config: TTSConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct TTSConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<String>, // "auto" | "on" | "off"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_language_text_normalization: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct VoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>
}

pub async fn tts_request(api_key: &str, request: TTSRequest) -> Result<Response, Error> {


    info!("Creating TTS request.");

    let voice_id = "ehhPcCa4Hn43ZNu1lHRk";

    let url = format!("https://api.elevenlabs.io/v1/text-to-speech/{}", voice_id);

    let client = Client::new();

    let payload = client
        .post(url)
        .header("xi-api-key", api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .build()?;

    info!("Sending request...");
    info!("Request content: {:?}", payload);

    match client.execute(payload).await {
        Ok(res) => return res.error_for_status(),
        Err(err) => {
            error!("Unable to send response");
            return Err(err)
        }
    }
}
