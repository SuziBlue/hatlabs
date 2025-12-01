
use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};
use hound::{WavSpec, WavWriter};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use tts_magic::tts::{AlignedChar, AlignmentChunk, TtsOutput};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlignedCharOld {
    pub ch: String,
    pub start_ms: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlignmentChunkOld {
    pub items: Vec<AlignedCharOld>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TtsOutputOld {
    pub audio: Vec<f32>,
    pub alignment: AlignmentChunkOld,
}

fn float_samples_to_wav_bytes(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut buffer = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut buffer, spec)?;

    for &sample in samples {
        let clamped = (sample * i16::MAX as f32)
            .clamp(i16::MIN as f32, i16::MAX as f32)
            as i16;
        writer.write_sample(clamped)?;
    }
    writer.finalize()?;

    Ok(buffer.into_inner())
}

fn main() -> Result<()> {
    // Connect to existing DB
    let conn = Connection::open("tts.db")?;

    // 1. Create new table (if not created yet)
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS tts_entries_new (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            alignment TEXT NOT NULL,
            audio BLOB NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );",
    )?;

    // 2. Query old data
    let mut stmt = conn.prepare("SELECT id, tts_result_json, created_at FROM tts_entries")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    // 3. Process each row: deserialize JSON, convert audio, serialize alignment
    for row in rows {
        let (id, tts_result_json, created_at) = row?;

        // Deserialize stored JSON into TtsOutput
        let tts_output_old: TtsOutputOld = serde_json::from_str(&tts_result_json)
            .map_err(|e| anyhow!("Failed to deserialize tts_result_json for id {}: {}", id, e))?;

        let alignment_new = AlignmentChunk { 
            items: tts_output_old.alignment.items.iter().map(|s| {
                    AlignedChar{
                        ch: s.ch.chars().next().unwrap(),
                        start_ms: s.start_ms,
                        duration_ms: s.duration_ms,
                    }
                }
            ).collect::<Vec<AlignedChar>>(),
        };
        let tts_output_new = TtsOutput {
            audio: tts_output_old.audio.clone(),
            alignment: alignment_new,
        };
        // Convert audio samples to wav bytes (assume 22050Hz or your actual sample rate)
        let wav_bytes = float_samples_to_wav_bytes(&tts_output_new.audio, 24000)?;

        // Serialize alignment to JSON string
        let alignment_json = serde_json::to_string(&tts_output_new.alignment)?;

        // Insert into new table
        conn.execute(
            "INSERT INTO tts_entries_new (alignment, audio, created_at) VALUES (?1, ?2, ?3)",
            params![alignment_json, wav_bytes, created_at],
        )?;
    }

    // 4. (Optional) Replace old table with new table
    conn.execute_batch(
        "DROP TABLE tts_entries;
         ALTER TABLE tts_entries_new RENAME TO tts_entries;",
    )?;

    println!("Migration complete!");

    Ok(())
}
