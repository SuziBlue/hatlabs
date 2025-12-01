
use base64::prelude::*;
use reqwest::Client;
use serde_json::json;
use std::{env, fs};
use anyhow::Result;
use log::{debug};
use crate::screen_capture::capture_screen;

pub struct VisionClient {
    api_key: String,
    client: Client,
}

impl VisionClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY")?;
        Ok(VisionClient {
            api_key,
            client: Client::new(),
        })
    }

    pub async fn describe_image(&self, image_data: &[u8]) -> Result<String> {
        let encoded_image = BASE64_STANDARD.encode(image_data);
        let image_data_uri = format!("data:image/png;base64,{}", encoded_image);

        let payload = json!({
            "model": "gpt-4.1",
            "messages": [
                {
                    "role": "user",
                    "content": [
                        { "type": "text", "text": "Describe what's happening in this image." },
                        {
                            "type": "image_url",
                            "image_url": {
                                "url": image_data_uri
                            }
                        }
                    ]
                }
            ],
            "max_tokens": 500
        });

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        let body = response.text().await?;
        debug!("OpenAI response: {}", body);

        #[derive(serde::Deserialize)]
        struct OpenAiResponse {
            choices: Vec<Choice>,
        }

        #[derive(serde::Deserialize)]
        struct Choice {
            message: Message,
        }

        #[derive(serde::Deserialize)]
        struct Message {
            content: String,
        }

        let parsed: OpenAiResponse = serde_json::from_str(&body)?;
        Ok(parsed.choices.first().map(|c| c.message.content.clone()).unwrap_or_default())
    }
}

