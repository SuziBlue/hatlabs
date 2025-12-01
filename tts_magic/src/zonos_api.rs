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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TTSConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaking_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_iso_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmax: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<Emotion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitchStd: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_noised: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Emotion {
    pub happiness: Option<f32>, // Default: 0.6
    pub sadness: Option<f32>,   // Default: 0.05
    pub disgust: Option<f32>,   // Default: 0.05
    pub fear: Option<f32>,      // Default: 0.05
    pub surprise: Option<f32>,  // Default: 0.05
    pub anger: Option<f32>,     // Default: 0.05
    pub other: Option<f32>,     // Default: 0.5
    pub neutral: Option<f32>,   // Default: 0.6
}

impl TTSRequestBuilder {
    pub fn new() -> Self {
        Self {
            tts_config: TTSConfig::default(),
        }
    }

    // Setters for optional fields (optional but can be added for convenience)
    pub fn speaker_audio(mut self, speaker_audio: String) -> Self {
        self.tts_config.speaker_audio = Some(speaker_audio);
        self
    }

    pub fn speaking_rate(mut self, speaking_rate: f32) -> Self {
        self.tts_config.speaking_rate = Some(speaking_rate);
        self
    }

    pub fn language_iso_code(mut self, language_iso_code: String) -> Self {
        self.tts_config.language_iso_code = Some(language_iso_code);
        self
    }

    pub fn mime_type(mut self, mime_type: String) -> Self {
        self.tts_config.mime_type = Some(mime_type);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.tts_config.model = Some(model);
        self
    }

    pub fn fmax(mut self, fmax: f32) -> Self {
        self.tts_config.fmax = Some(fmax);
        self
    }

    pub fn emotion(mut self, emotion: Emotion) -> Self {
        self.tts_config.emotion = Some(emotion);
        self
    }

    pub fn pitchStd(mut self, pitchStd: f32) -> Self {
        self.tts_config.pitchStd = Some(pitchStd);
        self
    }

    pub fn speaker_noised(mut self, speaker_noised: bool) -> Self {
        self.tts_config.speaker_noised = Some(speaker_noised);
        self
    }

    pub fn build<T: Into<String>>(&self, text: T) -> TTSRequest {
        let request = TTSRequest { text: text.into(), config: self.tts_config.clone() };
        request
    }
}

pub struct TTSRequestBuilder {
    tts_config: TTSConfig,
}

pub async fn tts_request(api_key: &str, request: TTSRequest) -> Result<Response, Error> {
    let url = "http://api.zyphra.com/v1/audio/text-to-speech";

    let client = Client::new();

    let request = client
        .post(url)
        .header("X-API-Key", api_key)
        .header("Content-Type", "application/json")
        .json(&request)
        .build()?;

    println!("Request content: {:?}", request);

    let response = client.execute(request).await?.error_for_status();

    response
}
