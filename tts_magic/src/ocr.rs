use base64::Engine;
use reqwest::Client;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs;
use anyhow::Result;
use base64::prelude::*;
use log::{info, debug, error};

use crate::screen_capture::capture_screen;


#[derive(Debug, Serialize, Deserialize)]
pub struct OcrResponse {
    text_list: Vec<TextBox>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextBox {
    pub text: String,
    pub bbox: Vec<Vec<i32>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OcrRequest {
    image_base64: String,
}

pub struct OcrClient {
    base_url: String,
    client: Client,
}

impl OcrClient {
    pub fn new() -> Result<Self> {
        let base_url = env::var("OCR_URL")?;
        Ok(OcrClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        })
    }

    pub async fn send_image_data(&self, image_data: &[u8]) -> Result<Vec<TextBox>> {
        let encoded_image = BASE64_STANDARD.encode(image_data);

        let payload = OcrRequest {
            image_base64: encoded_image,
        };

        let url = format!("{}/ocr_base64", self.base_url);

        let response = self.client
            .post(&url)
            .json(&payload)
            .send().await?
            .error_for_status()?;

        info!("reqwest response: {:?}", response);

        let body = response.text().await?;

        info!("json response: {}", body);

        let ocr_response: Vec<TextBox> = serde_json::from_str(&body)?;

        Ok(ocr_response)
    }
}

pub async fn run_ocr() -> Result<Vec<TextBox>> {
    capture_screen()?;
    let ocr_client = OcrClient::new()?;
    let image_path = "./output/screenshot.png";
    let image_bytes = fs::read(image_path)?;

    debug!("Image bytes loaded: {:?}", image_bytes);
    
    ocr_client.send_image_data(&image_bytes).await
}

