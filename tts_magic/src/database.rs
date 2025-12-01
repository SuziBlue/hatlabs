
use rusqlite::{params, Connection, OptionalExtension, Result};
use chrono::Utc;
use crate::tts::TtsOutput;
use hound; 
use std::io::Cursor;

#[derive(Debug)]
pub struct TtsEntry {
    pub id: i32,
    pub alignment_json: String,
    pub audio_wav: Vec<u8>,
    pub created_at: String,
}

#[derive(Debug)]
pub struct TtsDatabase {
    conn: Connection,
}



impl TtsDatabase {
    // Updated schema creation
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tts_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                alignment TEXT NOT NULL,
                audio BLOB NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn insert_tts_result(&self, result: &TtsOutput) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        // 1. Serialize alignment
        let alignment_json = serde_json::to_string(&result.alignment)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // 2. Convert f32 audio samples to WAV bytes
        let wav_data = f32_samples_to_wav_bytes(&result.audio)?;

        // 3. Insert into DB
        self.conn.execute(
            "INSERT INTO tts_entries (alignment, audio, created_at)
             VALUES (?1, ?2, ?3)",
            params![alignment_json, wav_data, now],
        )?;

        Ok(())
    }
}

fn f32_samples_to_wav_bytes(samples: &[f32]) -> Result<Vec<u8>, rusqlite::Error> {
    let mut buffer = Cursor::new(Vec::new());
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 24000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::new(&mut buffer, spec)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    for &sample in samples {
        let clamped = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(clamped)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    }

    writer.finalize()
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

    Ok(buffer.into_inner())
}
