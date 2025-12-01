
use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, Mutex};

use base64::engine::general_purpose;
use base64::Engine;
use futures::StreamExt;
use futures::SinkExt;
use log::error;
use log::info;
use log::debug;
use anyhow::{anyhow, Result};
use serde_json::Value;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use http::{HeaderName, HeaderValue};
use serde_json::json;
use crossbeam_channel::{unbounded, bounded, Sender, Receiver};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};

pub mod tts_providers;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlignedChar {
    pub ch: char,
    pub start_ms: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct AlignmentChunk {
    pub items: Vec<AlignedChar>,
}



impl<'de> Deserialize<'de> for AlignmentChunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct AlignmentChunkRaw {
            #[serde(rename = "chars")]
            chars: Vec<char>,
            #[serde(rename = "charStartTimesMs")]
            start_times: Vec<u64>,
            #[serde(rename = "charDurationsMs")]
            durations: Vec<u64>,
        }

        let raw = AlignmentChunkRaw::deserialize(deserializer)?;

        let len = raw.chars.len();
        if raw.start_times.len() != len || raw.durations.len() != len {
            return Err(de::Error::custom("Mismatched lengths in alignment fields"));
        }

        let items = raw
            .chars
            .into_iter()
            .zip(raw.start_times)
            .zip(raw.durations)
            .map(|((ch, start), dur)| AlignedChar {
                ch,
                start_ms: start,
                duration_ms: dur,
            })
            .collect();

        Ok(AlignmentChunk { items })
    }
}

impl AlignmentChunk {
    pub fn new() -> Self {
        AlignmentChunk { items: Vec::new() }
    }

    fn last_end_time_ms(&self) -> u64 {
        if let Some(last_char) = self.items.last()
        {
            last_char.start_ms + last_char.duration_ms
        } else {
            0
        }
    }

    fn offset(mut self, offset: u64) -> Self {
        self.items.iter_mut().for_each(|item| {
            item.start_ms += offset
        });
        self
    }

    fn extend_from(&mut self, chunk: AlignmentChunk) {
        let time_offset = self.last_end_time_ms();

        self.items.extend(chunk.offset(time_offset).items);
    }
}


impl IntoIterator for AlignmentChunk {
    type Item = AlignedChar;
    type IntoIter = std::vec::IntoIter<AlignedChar>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[derive(Debug)]
pub struct TTSHandle {
    sender: JoinHandle<()>,
    receiver: JoinHandle<()>,
    pub tts_tx: Sender<String>,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TtsOutput {
    pub audio: Vec<f32>,
    pub alignment: AlignmentChunk,
}

pub trait TtsModule {
    fn connect_input(&self) -> Sender<String>;
    fn connect_output(&self) -> Receiver<VecDeque<f32>>;    
    async fn join(self) -> Result<TtsOutput>;
}
