use crate::api::{TTSConfig, VoiceSettings};

impl Default for TTSConfig {
    fn default() -> Self {
        TTSConfig {
            model_id: Some("eleven_multilingual_v2".to_string()),
            voice_settings: Some(VoiceSettings::default()),
            language_code: None,
            seed: None,
            previous_text: None,
            next_text: None,
            apply_text_normalization: None,
            apply_language_text_normalization: None,
        }
    }
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self { 
            stability: None, 
            similarity_boost: None, 
            style: None, 
            use_speaker_boost: None,
            speed: None,
        }
    }
}

pub const CHANNEL_BUFFER_SIZE: usize = 32;
