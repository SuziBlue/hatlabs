use crate::api::TTSConfig;

impl Default for TTSConfig {
    fn default() -> Self {
        TTSConfig {
            speaker_audio: None,
            //speaking_rate: Some(15.0),
            //language_iso_code: Some("en-us".to_string()),
            //mime_type: Some("audio/mp3".to_string()),
            //model: Some("zonos-v0.1-hybrid".to_string()),
            speaking_rate: None,
            language_iso_code: None,
            mime_type: None,
            model: None,
            fmax: None,
            emotion: None,
            pitchStd: None,
            speaker_noised: None,
        }
    }
}

pub const CHANNEL_BUFFER_SIZE: usize = 32;
