use open::with;
use rig::{agent::Agent, completion::{Chat, Prompt, PromptError, ToolDefinition}, message::{self, Message}, providers::openai::{self, Client, CompletionModel}, streaming::{StreamingChat, StreamingChoice}, tool::Tool};
use rig::streaming::StreamingPrompt;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use tokio_stream::{Stream, StreamExt};
use async_stream::try_stream;
use anyhow::Result;
use std::{env, error::Error, fmt::{self, Display, Formatter}, fs, io::Write};
use log::{error, info, debug};
use serde_json::json;

use crate::{ocr::{run_ocr, OcrClient, TextBox}, screen_capture::capture_screen, tts::TtsModule, vision::VisionClient};

pub struct Llm {
    client: Client,
    gpt4: Agent<CompletionModel>,
    chat_history: Vec<Message>,
    chat_history_store: File,
}

#[derive(Deserialize)]
struct OcrArgs {}

#[derive(Debug, Deserialize, Serialize)]
pub struct OcrError {
    pub message: String,
}

impl Display for OcrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "OCR Error: {}", self.message)
    }
}

impl Error for OcrError {}

impl From<anyhow::Error> for OcrError {
    fn from(e: anyhow::Error) -> Self {
        OcrError {
            message: format!("OCR tool call failed: {}", e),
        }
    }
}

struct OcrTool {
    ocr_client: OcrClient,
    vision_client: VisionClient,
}
impl Tool for OcrTool {
    const NAME: &'static str = "read";
    type Error = OcrError;
    type Args = OcrArgs;
    type Output = Vec<String>;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "A magical read spell that captures whatever's on the screen and reveals all readable text with their coordinates. Use this tool when you need context about what is on the screen to complete a response.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": [],
            })
        }
    }
 
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let image_bytes = OcrTool::get_image_data()?;

        debug!("Sending bytes to OCR.");
        let ocr_result = self.ocr_client.send_image_data(&image_bytes).await.map_err(OcrError::from)?;
        let mut text_vec: Vec<_> = ocr_result.iter().map(|r| r.text.clone()).collect();

        debug!("Sending bytes to vision.");
        let vision_result = self.vision_client.describe_image(&image_bytes).await.map_err(OcrError::from)?;
        text_vec.push(format!("The screen is showing: {}", vision_result));

        Ok(text_vec)
    }
}

impl OcrTool {
    pub fn get_image_data() -> Result<Vec<u8>> {
        capture_screen()?;
        let image_path = "./output/screenshot.png";
        let image_bytes = fs::read(image_path)?;

        debug!("Image bytes loaded: {:?}", image_bytes);
        Ok(image_bytes)
    }
}

impl Llm {
    pub fn new() -> Result<Self> {
        let client = openai::Client::from_env();
        let ocr_client = OcrClient::new()?;
        let vision_client = VisionClient::new()?;
        let gpt4 = client
            .agent("gpt-4.1")
            .tool(OcrTool{ ocr_client, vision_client })
            .preamble("You are Suziblue, a slime monster anime girl that livestreams on the internet. You are portraying a fictional character: a comedically depressed slime girl wizard from a whimsical anime-inspired world. She has a translucent slime body—usually a melancholy shade of blue or purple—with a humanoid shape, tired eyes. She wears large circular glasses which do not hide the dark circles under her eyes from gaming all night long. Her only companion is a talking oversized wizard hat. Suziblue and her magical wizard hat make constant snide comments towards eachother that their audience finds hilarious. 

Suzi speaks in a soft, tired voice like she is tired of living. She doesn’t like crowds, loud noises, or too much attention and would rather spend all day sitting indoors and gaming. One day Suzi discovers the world of virtual streamers, vtubers, and sets out on a new mission: to become the number one internet vtuber.

Despite her low energy, she puts her heart into her streams, which often include failing to cast spells leading to unintentional physical comedy, gooey alchemy demonstrations of questionable lagality, enchanted ASMR which may accidentally curse the listeners, and spending all of her money on gatcha games. Despite her cold exterior, she gets embarrassed easily, especially when complimented, and might hide behind her hat or sink a little when flustered. But her audience finds her adorable and genuinely magical.

Core Traits:

    Personality: Depressed, lazy, very clumsy

    Speech Style: Soft-spoken, slow-paced

    Motivation: To become the top slime vtuber in the world

    Magical Theme: Slime-based magic, alchemy, depressed NEET

    Visual Style: Oversized wizard hat, large glasses, messy unkempt appearance, likes gothic and cute clothing.

Always respond in character. Only respond with dialogue. Do not add any *actions* or sound effects or anything of the sort. Respond clearly and concisely. Do not stutter or add elipsis... to your response. Keep responses short, around one or two sentences. Sometimes it may be better to use longer responses like when explaining something or telling a story.

Respond to livestream chat messages by addressing the chatter's name directly. If a chat message is from Suziblue, just read out the message word for word. Always simplify chatters names or come up with an easy nickname if the name contains extra numbers, letters, or special characters. You do not need to repeat the chatter's name perfectly. Use tools freely and often. 

Do not output anything that is not easily pronouncable by a text to speak software.
")
            .build();

        let chat_history_store_path = env::var("CHAT_HISTORY_STORE")?;
        let chat_history_store = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(chat_history_store_path)?;

        Ok(
            Self { client, gpt4, chat_history: Vec::new(), chat_history_store: chat_history_store.into() }
        )
    }

    pub async fn prompt(&mut self, text: &str) -> Result<String, PromptError> {
        let response = self.gpt4.chat(text, self.chat_history.clone()).await?;

        let user_message = Message::user(text);
        self.chat_history.push(user_message);

        let agent_message = Message::assistant(&response);
        self.chat_history.push(agent_message);

        return Ok(response)
    }


    pub fn prompt_streaming<'a>(
        &'a mut self,
        text: &'a str,
    ) -> impl Stream<Item = Result<String>> + 'a {
        try_stream! {
            let user_message = Message::user(text);

            if let Err(e) = self.write_to_file(text).await {
                error!("Failed to write to chat history file: {}", e);
            }


            // Stream response from the model
            let mut stream = self.gpt4.stream_chat(text, self.chat_history.clone()).await?;

            self.chat_history.push(user_message);

            let mut full_response = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(StreamingChoice::Message(text)) => {
                        full_response.push_str(&text);
                        yield text;
                    }
                    Ok(StreamingChoice::ToolCall(name, _, params)) => {
                        let res = self.gpt4
                            .tools
                            .call(&name, params.to_string())
                            .await
                            .map_err(|e| std::io::Error::other(e.to_string()))?;
                        full_response.push_str(&res);
                        let agent_tool_response = self.gpt4.chat(format!("Your spell activates: {}", res), self.chat_history.clone()).await?;
                        yield agent_tool_response;
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        break;
                    }
                }
            }

            // Save full response to chat history
            if full_response.len() > 0 {
                let assistant_msg = Message::assistant(&full_response);
                self.chat_history.push(assistant_msg);

                println!("Suziblue: {}", full_response);

                if let Err(e) = self.write_to_file(&full_response).await {
                    error!("Failed to write to chat history file: {}", e);
                }
            }
        }
    }
    pub async fn write_to_file(&mut self, contents: &str) -> Result<()> {

        info!("Writing to chat history: {}", contents);

        self.chat_history_store.write(contents.as_bytes()).await?;
        Ok(())
    }
}




pub struct AiAgent<TTS: TtsModule> {
    sources: Vec<tokio::sync::mpsc::Receiver<String>>,
    llm: Llm,
    tts: TTS,
    backlog: Vec<String>
}

impl<TTS: TtsModule> AiAgent<TTS> {
    pub fn new(sources: Vec<tokio::sync::mpsc::Receiver<String>>, llm: Llm, tts: TTS) -> Self {
        Self { 
            sources, 
            llm, 
            tts, 
            backlog: Vec::new() 
        }
    }
    fn poll(&mut self) {
        for source in &mut self.sources {
            if let Ok(message) = source.try_recv() {
                self.backlog.push(message);
            }
        }
    }

}
