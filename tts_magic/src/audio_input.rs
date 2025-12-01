use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine};
use rodio::{cpal::{self, traits::{HostTrait, StreamTrait}, SampleFormat}, DeviceTrait};
use anyhow::Result;
use anyhow::anyhow;
use tokio::sync::broadcast::channel;


pub struct InputHandler {
    input_audio_stream: cpal::Stream,
    input_device: cpal::Device,
    buffer_clone: Arc<Mutex<Vec<i16>>>,
}

impl InputHandler {

    pub fn try_new() -> Result<Self> {
        let host = cpal::default_host();
        let input_device = host.default_input_device().ok_or(anyhow!("No default input device."))?;
        // Audio capture
        let supported_formats = vec![SampleFormat::I16, SampleFormat::F32, SampleFormat::U16];
        let config = input_device.supported_input_configs()?
            .filter(|conf| {supported_formats.contains(&conf.sample_format())})
            .next()
            .ok_or(anyhow!("No supported formats."))?
            .with_sample_rate(cpal::SampleRate(16000));

        // Shared audio buffer
        let audio_buffer: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = audio_buffer.clone();

        let stream = match config.sample_format() {
            SampleFormat::I16 => {
                input_device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _| {
                        let mut buffer = audio_buffer.lock().unwrap();
                        buffer.extend_from_slice(data);
                    },
                    err_fn,
                    None,
                )?
            }
            SampleFormat::F32 => {
                input_device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _| {
                        let mut buffer = audio_buffer.lock().unwrap();
                        buffer.extend(data.iter().map(|s| {
                            let s_clamped = s.clamp(-1.0, 1.0);
                            (s_clamped * i16::MAX as f32) as i16
                        }));
                    },
                    err_fn,
                    None,
                )?
            }
            SampleFormat::U16 => {
                input_device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _| {
                        let mut buffer = audio_buffer.lock().unwrap();
                        buffer.extend(data.iter().map(|s| {
                            // Convert u16 (0–65535) to i16 (-32768–32767)
                            let s = *s as i32 - 32768;
                            s as i16
                        }));
                    },
                    err_fn,
                    None,
                )?
            }
            _ => {
                return Err(anyhow!("Unknown sample format. Only i16 f32 u16 is supported."));
            }
        };
        stream.play()?;
        println!("Recording and streaming...");

        Ok(Self { input_audio_stream: stream, input_device, buffer_clone })
    }

    pub async fn stream_b64(&mut self) -> Result<tokio::sync::broadcast::Sender<String>> {

        let buffer_clone = self.buffer_clone.clone();
        //
        let (audio_tx, _audio_rx) = channel::<String>(100);
        let cloned_tx = audio_tx.clone();
        // Spawn a task to periodically send audio chunks
        let _sender = tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;

                let mut chunk: Vec<i16> = vec![];
                {
                    let mut buffer = buffer_clone.lock().unwrap();
                    if buffer.len() >= 3200 {
                        chunk = buffer.drain(..3200).collect(); // ~100ms @ 24kHz mono
                    }
                }

                if !chunk.is_empty() {
                    let bytes: Vec<u8> = chunk.iter().flat_map(|s| s.to_le_bytes()).collect();
                    let b64_audio = general_purpose::STANDARD.encode(&bytes);
                    cloned_tx.send(b64_audio);
                }
            }
        });

        Ok(audio_tx)
    }
}


fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
