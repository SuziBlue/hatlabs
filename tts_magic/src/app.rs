use crate::ocr::OcrClient;
use crate::sound_player::SoundPlayer;
use crate::stt;
use crate::tts::tts_providers::eleven::ElevenTtsModule;
use crate::tts::TtsModule;
use crate::user_input::InputHandler;
use crate::config::CHANNEL_BUFFER_SIZE;
use crate::llm::Llm;
use crate::youtube::api::get_active_livestream_chat_id;
use crate::youtube::api::get_livestreams;
use crate::youtube::api::get_youtube_token;
use crate::youtube::api::spawn_youtube_chat_stream;
use crate::screen_capture::capture_screen;
use crate::youtube::livechatmessages::LiveChatMessage;
use crate::database::TtsDatabase;

use crossbeam_channel::bounded;
use crossbeam_channel::{Sender, Receiver};
use dotenv::dotenv;
use log::debug;
use log::error;
use log::info;
use tokio::time::{Duration, sleep};
use tokio_stream::StreamExt;
use tokio::select;
use std::env;
use std::error::Error;
use std::fs;
use std::time::Instant;
use anyhow::Result;


pub struct App {
    action_receiver: Receiver<AppActions>,
    sound_player: SoundPlayer,
    llm: Llm,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum AppActions {
    DisplayDevices,
    TestOutputDevice,
    QuitApp,
    TestInputDevice,
    StartStream,
    ListLivestreams,
    TestOCR,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        info!("Loading .env");
        dotenv()?;
        
        info!("Starting input handler.");
        let (user_input_tx, user_input_rx) = bounded(CHANNEL_BUFFER_SIZE);

        tokio::spawn(async move {
            InputHandler::run(user_input_tx).await;
        });

        info!("Starting sound player.");
        let sound_player = SoundPlayer::try_default()?;

        info!("Starting LLM.");
        let llm = Llm::new()?;

        info!("Startup finished.");
        Ok(Self {
            llm,
            action_receiver: user_input_rx,
            sound_player,
        })
    }
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        
        loop {
            let next_action = self.action_receiver.recv()?;
            match next_action {
                AppActions::QuitApp => break,
                AppActions::TestOutputDevice => {
                    if let Err(err) = self.sound_player.test_output_device() {
                        println!("Cannot use device. Err: {err}");
                    }
                },
                AppActions::DisplayDevices => {
                    if let Some(devices) = SoundPlayer::list_output_devices() {
                        println!("Output Devices:");
                        for device in devices {
                            println!("  {}", device);
                        }
                    } else {
                        println!("No output devices found.")
                    }
                },
                AppActions::TestInputDevice => {
                    info!("Testing input device.");
                    let _ = self.sound_player.input_loopback();
                }
                AppActions::StartStream => {

                    //let rx = match self.sound_player.run_stt().await {
                    let mut input_audio_handler = match crate::audio_input::InputHandler::try_new() {
                        Ok(t) => t,
                        Err(e) => {
                            error!("Can't start input audio handler: {}", e);
                            continue
                        }
                    };
                    let input_audio = match input_audio_handler.stream_b64().await {
                        Ok(t) => t,
                        Err(e) => {
                            error!("Can't start audio stream: {}", e);
                            continue
                        }
                    };
                    let rx = match stt::start(input_audio).await {
                        Ok(t) => t,
                        Err(e) => {
                            error!("Can't start stt provider: {}", e);
                            continue
                        }
                    };
                    println!("Channel open. Awaiting transcription.");

                    let token = get_youtube_token().await;
                    let chat_rx = match get_active_livestream_chat_id(token.clone()).await {
                        Err(e) => {
                            error!("Can't find active livestream: {}", e);
                            None
                        }
                        Ok(None) => {
                            error!("No active livestreams.");
                            None
                        }
                        Ok(Some(chat_id)) => {
                            Some(spawn_youtube_chat_stream(token, chat_id))
                        }
                    };
                    println!("Chat connected.");
                    
                    self.run_loop(rx, chat_rx).await;
                }
                AppActions::ListLivestreams => {
                    let token = get_youtube_token().await;
                    if let Ok(livestreams) = get_livestreams(token).await {
                        for livestream in livestreams {
                            println!("Livestream: {:?}", livestream);
                        }
                    } else {
                        eprintln!("Failed to get livestreams.")
                    }
                }
                AppActions::TestOCR => {
                    capture_screen()?;
                    let ocr_client = OcrClient::new()?;
                    let image_path = "./output/screenshot.png";
                    let image_bytes = fs::read(image_path)?;

                    debug!("Image bytes loaded: {:?}", image_bytes);
                    
                    match ocr_client.send_image_data(&image_bytes).await {
                        Ok(ocr_response) =>  {
                            println!("OCR Response: {:?}", ocr_response);
                        } 
                        Err(e) => {
                            error!("Unable to use OCR service: {}", e);
                        }
                    }
                }
                _ => continue,
            }
        }

        Ok(())
    }

    async fn run_loop(
        &mut self,
        mut rx: tokio::sync::mpsc::Receiver<String>, // STT transcription
        mut chat_rx: Option<tokio::sync::mpsc::Receiver<LiveChatMessage>>, // YouTube chat
    ) {
        println!("Channel open. Awaiting transcription.");
        println!("Chat connected.");

        let idle_duration = Duration::from_secs(40);
        let mut last_input = Instant::now();
        println!("Idle timer started.");

        loop {
            let timeout = idle_duration
                .checked_sub(last_input.elapsed())
                .unwrap_or(Duration::from_secs(0));
            let timeout_sleep = sleep(timeout);
            tokio::pin!(timeout_sleep);

            select! {
                maybe_transcription = rx.recv() => {
                    match maybe_transcription {
                        Some(transcription) => {
                            println!("App received transcription: {}", transcription);
                            let prompt = format!("Your talking hat says: {}", transcription);
                            if let Err(e) = self.handle_prompt(&prompt).await {
                                error!("App: Error while handling prompt {}", e);
                            }
                        }
                        None => {
                            println!("STT channel closed.");
                            break;
                        }
                    }
                    last_input = Instant::now();
                }
            
                Some(message) = async {
                    if let Some(chat_rx) = &mut chat_rx {
                        chat_rx.recv().await
                    } else {
                        None
                    }
                }, if chat_rx.is_some() => {
                    let prompt = format!(
                        "A message has arrived from the livestream chat. The chatter {} says: {}",
                        message.authorDetails.displayName,
                        message.snippet.displayMessage
                    );
                    if let Err(e) = self.handle_prompt(&prompt).await {
                        error!("App: Error while handling prompt {}", e);
                    }
                    last_input = Instant::now();
                }
            
                _ = &mut timeout_sleep => {
                    println!("No input received for {:?}. Prompting LLM...", idle_duration);
                    let prompt = "Nothing happened, so you decide to continue your previous thought.";
                    if let Err(e) = self.handle_prompt(&prompt).await {
                        error!("App: Error while handling prompt {}", e);
                    }
                    last_input = Instant::now();
                }
            }
        }

        println!("All channels closed.");
    }

    async fn handle_prompt(&mut self, prompt: &str) -> Result<()> {
        info!("Creating TTS stream.");
        let voice_id = env::var("ELEVEN_VOICE_ID")?;
        let api_key = env::var("ELEVEN_API_KEY")?;
        let model_id = env::var("ELEVEN_MODEL_ID")?;
        let tts_module = ElevenTtsModule::try_new(voice_id, api_key, model_id).await?;
        let tts_rx = tts_module.connect_output();
        let tts_tx = tts_module.connect_input();
        self.sound_player.play_stream2(tts_rx)?;

        info!("Sending prompt '{}' to LLM.", prompt);
        let mut response_stream = Box::pin(self.llm.prompt_streaming(prompt));

        while let Some(text_chunk) = response_stream.next().await {
            match text_chunk {
                Ok(text) => {
                    if let Err(e) = tts_tx.send(text) {
                        eprintln!("Failed to send text to TTS: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error in prompt stream: {}", e);
                    break;
                }
            }
        }
        drop(tts_tx);

        info!("App: Waiting for TTS to finish.");
        let tts_result = tts_module.join().await?;

        info!("App: Inserting TTS output into database.");
        let db = TtsDatabase::new("./tts.db")?;
        db.insert_tts_result(&tts_result)?;

        Ok(())
    }
}
