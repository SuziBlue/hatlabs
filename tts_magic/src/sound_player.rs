use log::{error, info};
use oauth2::http::HeaderName;
use reqwest::header::AUTHORIZATION;
use rodio::cpal::traits::{HostTrait, StreamTrait};
use rodio::cpal::{Host, SampleRate, Stream, StreamConfig};
use rodio::source::SineWave;
use rodio::{cpal, Decoder, Device, DeviceTrait, OutputStream, OutputStreamHandle, PlayError, Sink, Source, StreamError};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use std::collections::VecDeque;
use std::io::{BufReader, Read, Seek};
use std::time::Duration;
use anyhow::Error;
use anyhow::anyhow;
use anyhow::Result;
use ringbuf::{
    traits::{Consumer, Producer, Split},
    HeapRb,
};
use base64::{engine::general_purpose, Engine as _};
use cpal::{traits::*, SampleFormat};
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::sync::{self, Arc, Mutex};
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crossbeam_channel::{bounded, Receiver, Sender};

struct STTHandle {
    audio_stream: Stream,
    sender: JoinHandle<()>,
    receiver: JoinHandle<()>,
}

pub struct SoundPlayer {
    stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    host: Host,
    input_device: Device,
    output_device: Device,
    stt_handle: Option<STTHandle>,
    output_stream: Option<Stream>,
}

impl SoundPlayer {
    pub fn try_default() -> Result<Self, Error> {
        info!("Creating output stream with default device.");
        let (stream, stream_handle) = OutputStream::try_default()?;
        let host = cpal::default_host();
        let input_device = host.default_input_device().ok_or(anyhow!("No default input device."))?;
        let output_device = host.default_output_device().ok_or(anyhow!("No default output device."))?;

        info!("Found input device: {}", input_device.name()?);
        
        Ok(Self {
            stream,
            stream_handle,
            host,
            input_device,
            output_device,
            stt_handle: None,
            output_stream: None,
        })
    }

    pub fn play_audio<R>(&self, audio_data: R)
    where 
        R: Read + Seek + Sync + Send + 'static,
    {
        let sink = match Sink::try_new(&self.stream_handle) {
            Ok(s) => s,
            Err(err) => {
                error!("Failed to create audio sink: {}", err);
                return
            }
        };

        let source = match Decoder::new(audio_data) {
            Ok(s) => s,
            Err(err) => {
                error!("Failed to decode audio data: {}", err);
                return
            }
        };

        sink.append(source);
        sink.sleep_until_end();
    }
    pub fn play_audio_from_file(&self, file: impl Read + Seek + Send + Sync + 'static) {
        let audio_data = BufReader::new(file);
        self.play_audio(audio_data);
    }

    pub fn get_output_device_by_name(target_name: &str) -> Option<Device> {

        let host = cpal::default_host();

        for device in host.output_devices().ok()? {
            if let Ok(name) = device.name() {
                if name == target_name {
                    return Some(device);
                }
            }
        }

        None
    }

    pub fn test_output_device(&self) -> Result<(), PlayError> {
        let sink = Sink::try_new(&self.stream_handle)?;

        let source = SineWave::new(440.0)
            .take_duration(Duration::from_secs_f32(5.0))
            .amplify(0.4);

        sink.append(source);
        sink.sleep_until_end();

        Ok(())
    }

    pub fn list_output_devices() -> Option<Vec<String>>{
        let devices = cpal::default_host().output_devices().ok()?;
        let device_names: Vec<String> = devices.filter_map(|device| device.name().ok()).collect();
        Some(device_names)
    }

    pub fn input_loopback(&self) -> Result<()> {

        let mut supported_configs_range = self.input_device.supported_input_configs()?;
        let config: StreamConfig = supported_configs_range.next().ok_or(anyhow!("No supported configs."))?.with_max_sample_rate().into();

        // Create a delay in case the input and output devices aren't synced.
        let latency = 1_000.0;
        let latency_frames = (latency / 1_000.0) * config.sample_rate.0 as f32;
        let latency_samples = latency_frames as usize * config.channels as usize;

        // The buffer to share samples
        let ring = HeapRb::<f32>::new(latency_samples * 2);
        let (mut producer, mut consumer) = ring.split();

        // Fill the samples with 0.0 equal to the length of the delay.
        for _ in 0..latency_samples {
            // The ring buffer has twice as much space as necessary to add latency here,
            // so this should never fail
            producer.try_push(0.0).unwrap();
        }

        let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut output_fell_behind = false;
            for &sample in data {
                if producer.try_push(sample).is_err() {
                    output_fell_behind = true;
                }
            }
            if output_fell_behind {
                eprintln!("output stream fell behind: try increasing latency");
            }
        };

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut input_fell_behind = false;
            for sample in data {
                *sample = match consumer.try_pop() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
            }
            if input_fell_behind {
                eprintln!("input stream fell behind: try increasing latency");
            }
        };

        // Build streams.
        println!(
            "Attempting to build both streams with f32 samples and `{:?}`.",
            config
        );
        let input_stream = self.input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;
        let output_stream = self.output_device.build_output_stream(&config, output_data_fn, err_fn, None)?;
        println!("Successfully built streams.");

        // Play the streams.
        println!(
            "Starting the input and output streams with `{}` milliseconds of latency.",
            latency
        );
        input_stream.play()?;
        output_stream.play()?;

        // Run for 3 seconds before closing.
        println!("Playing for 3 seconds... ");
        std::thread::sleep(std::time::Duration::from_secs(3));
        drop(input_stream);
        drop(output_stream);
        println!("Done!");
        Ok(())

    }

    pub async fn run_stt(&mut self) -> Result<Receiver<String>> {

        // Connect to WebSocket
        let url = "wss://api.openai.com/v1/realtime?intent=transcription";
        let api_key = std::env::var("OPENAI_API_KEY")?;

        // Build request with Authorization header
        let mut request = url.into_client_request()?;
        request
            .headers_mut()
            .insert(AUTHORIZATION, format!("Bearer {}", api_key).parse()?);

        // Add required beta header
        let beta_header = HeaderName::from_static("openai-beta");
        request.headers_mut().insert(
            beta_header,
            "realtime=v1".parse()?,
        );

        // Connect with custom request
        let (ws_stream, _) = connect_async(request).await?;
        let (mut write, mut read) = ws_stream.split();

        let config_msg = json!(
            {
              "type": "transcription_session.update",
              "session": {
                "input_audio_format": "pcm16",
                "input_audio_noise_reduction": null,
                "input_audio_transcription": {
                  "model": "gpt-4o-transcribe",
                  "prompt": "",
                  "language": "en"
                },
                "turn_detection": {
                  "type": "server_vad",
                  "threshold": 0.5,
                  "prefix_padding_ms": 300,
                  "silence_duration_ms": 2000
                }
              }
            }
        );
        write.send(Message::Text(config_msg.to_string().into())).await?;

        // Audio capture
        let supported_formats = vec![SampleFormat::I16, SampleFormat::F32, SampleFormat::U16];
        let config = self.input_device.supported_input_configs()?
            .filter(|conf| {supported_formats.contains(&conf.sample_format())})
            .next()
            .ok_or(anyhow!("No supported formats."))?
            .with_sample_rate(cpal::SampleRate(16000));

        // Shared audio buffer
        let audio_buffer: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = audio_buffer.clone();

        let stream = match config.sample_format() {
            SampleFormat::I16 => {
                self.input_device.build_input_stream(
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
                self.input_device.build_input_stream(
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
                self.input_device.build_input_stream(
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

        // Spawn a task to periodically send audio chunks
        let sender = tokio::spawn(async move {
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
                    let msg = json!({
                        "type": "input_audio_buffer.append",
                        "audio": b64_audio
                    });

                    if let Err(e) = write.send(Message::Text(msg.to_string().into())).await {
                        eprintln!("WebSocket send error: {}", e);
                        break;
                    }
                }
            }
        });


        let (tx, rx): (Sender<String>, Receiver<String>) = bounded(10);

        // Spawn a task to receive and print transcriptions
        let receiver = tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<serde_json::Value>(&text) {
                            Ok(json) => {
                                info!("🗣️ Whisper says:\n{}", serde_json::to_string_pretty(&json).unwrap());
                                if json["type"] == "conversation.item.input_audio_transcription.completed" {
                                    let transcript = json["transcript"].to_string(); 
                                    println!("STT sending transcript to app {}", transcript);
                                    if let Err(e) = tx.send(transcript) {
                                        error!("Error while sending transcript: {}", e);
                                    }
                                }
                            },
                            Err(_) => info!("🗣️ Whisper says (raw): {}", text),
                        }
                    }
                    Ok(other) => info!("Other WS message: {:?}", other),
                    Err(e) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });

        self.stt_handle = Some(STTHandle{
            audio_stream: stream,
            sender,
            receiver,
        });

        Ok(rx)
    }
    pub async fn play_stream(&mut self) -> Result<Arc<Mutex<VecDeque<f32>>>> {
        let mut supported_configs_range = self.output_device.supported_output_configs()?;
        let config: StreamConfig = supported_configs_range
            .next()
            .ok_or(anyhow!("No supported configs."))?
            .with_sample_rate(SampleRate(24000))
            .into();

        // Create the shared buffer (dynamically growable)
        let audio_buffer = Arc::new(Mutex::new(VecDeque::<f32>::new()));

        // Clone buffer handle for use in the audio callback
        let buffer_for_callback = Arc::clone(&audio_buffer);

        let output_data_fn = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let mut buffer = buffer_for_callback.lock().unwrap();

            for sample in data {
                *sample = if let Some(s) = buffer.pop_front() {
                    s
                } else {
                    0.0
                };
            }
        };

        // Build and start the output stream
        let output_stream = self.output_device.build_output_stream(&config, output_data_fn, err_fn, None)?;
        self.output_stream = Some(output_stream);

        println!("Audio stream started with dynamic buffer.");
        Ok(audio_buffer)
    }
    pub fn play_stream2(&mut self, sound_receiver: Receiver<VecDeque<f32>>) -> Result<()> {
        let mut supported_configs_range = self.output_device.supported_output_configs()?;
        let config: StreamConfig = supported_configs_range
            .next()
            .ok_or(anyhow!("No supported configs."))?
            .with_sample_rate(SampleRate(24000))
            .into();

        let shared_buffer = Arc::new(Mutex::new(VecDeque::new()));
        let buffer_clone = Arc::clone(&shared_buffer);

        tokio::spawn(async move {
            while let Ok(new_data) = sound_receiver.recv() {
                info!("Stream Player: Received new data.");
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend(new_data);
            }
        });


        let output_data_fn = {
            let buffer = Arc::clone(&shared_buffer);
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut buf = buffer.lock().unwrap();
                info!("Stream Player: Appending to buffer.");
                for sample in data.iter_mut() {
                    *sample = buf.pop_front().unwrap_or(0.0);
                }
            }
        };


        // Build and start the output stream
        let output_stream = self.output_device.build_output_stream(&config, output_data_fn, err_fn, None)?;
        output_stream.play()?;
        self.output_stream = Some(output_stream);

        println!("Audio stream started with dynamic buffer.");
        Ok(())
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
