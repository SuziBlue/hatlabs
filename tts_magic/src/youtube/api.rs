use super::livestream::{LiveBroadcast, LiveBroadcastListResponse};
use super::livechatmessages::{LiveChatMessage, LiveChatMessageListResponse};
use chrono::{DateTime, Utc};
use log::{error, info};
use oauth2::TokenResponse;
use reqwest::{Client, Response};
use tokio::task;
use std::error::Error;
use anyhow::{Context, Result};

use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, RefreshToken, RevocationUrl, Scope, StandardTokenResponse,
    TokenUrl,
};
use oauth2::reqwest;
use open;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::{Duration, SystemTime};
use std::{env, fs};
use url::Url;

pub type YoutubeToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Serialize, Deserialize)]
struct YoutubeTokenJar {
    youtube_token: YoutubeToken,
    expiration_time: SystemTime,
}

pub async fn get_youtube_token() -> YoutubeToken {
    if let Ok(token_jar) = load_token() {
        println!("Token loaded, checking expiration time.");
        if token_jar.expiration_time > SystemTime::now() {
            println!("Token is still valid.");
            return token_jar.youtube_token;
        }
        println!("Token is expired.");
        println!("Attempting to refresh token.");
        match refresh_youtube_token(
            token_jar
                .youtube_token
                .refresh_token()
                .expect("Malformed token"),
        ).await {
            Ok(token) => return token,
            Err(e) => {
                println!("Token refresh failed, requesting new token. {}", e);
                let token = get_new_youtube_token().await;
                let expiration_time = SystemTime::now() + token.expires_in().expect("Malformed token");
                let token_jar = YoutubeTokenJar {
                    youtube_token: token,
                    expiration_time: expiration_time,
                };
                save_token(&token_jar).expect("Failed to save token");
                return token_jar.youtube_token
            }
        }
    }
    println!("Failed to load token, requesting new token.");
    let token = get_new_youtube_token().await;
    let expiration_time = SystemTime::now() + token.expires_in().expect("Malformed token");
    let token_jar = YoutubeTokenJar {
        youtube_token: token,
        expiration_time: expiration_time,
    };
    save_token(&token_jar).expect("Failed to save token");
    token_jar.youtube_token
}

fn save_token(token: &YoutubeTokenJar) -> std::io::Result<()> {
    let json = serde_json::to_string(token)?;
    let token_path = env::var("TOKEN_PATH")
        .expect("Token path not found in .env")
        .to_string();
    fs::write(token_path, json)?;
    Ok(())
}

fn load_token() -> std::io::Result<YoutubeTokenJar> {
    let path = env::var("TOKEN_PATH")
        .expect("Token path not found in .env")
        .to_string();
    let string = fs::read_to_string(path)?;
    let token: YoutubeTokenJar = serde_json::from_str(&string)?;
    Ok(token)
}

pub async fn refresh_youtube_token(refresh_token: &RefreshToken) -> Result<YoutubeToken> {
    let client_id = ClientId::new(
        env::var("YOUTUBE_CLIENT_ID")
            .context("Client ID not found in environment variables")?,
    );
    let client_secret = ClientSecret::new(
        env::var("YOUTUBE_CLIENT_SECRET")
            .context("Client secret not found in environment variables")?,
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .context("Invalid authorization endpoint URL")?;
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .context("Invalid token endpoint URL")?;

    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url);

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build client");

    let token_result = client
        .exchange_refresh_token(refresh_token)
        .request_async(&http_client)
        .await
        .context("Failed to exchange refresh token")?;

    Ok(token_result)
}

async fn get_new_youtube_token() -> YoutubeToken {
    let client_id = ClientId::new(
        env::var("YOUTUBE_CLIENT_ID")
            .expect("Client ID not found in .env")
            .to_string(),
    );
    let client_secret = ClientSecret::new(
        env::var("YOUTUBE_CLIENT_SECRET")
            .expect("Client secret not found in .env")
            .to_string(),
    );

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
        .expect("Invalid token endpoint URL");

    let redirect_url = RedirectUrl::new("http://localhost:3000/oauth2callback".to_string())
        .expect("Invalid redirect URL");
    let revocation_url = RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
        .expect("Invalid revocation endpoint URL");

    // Set up the client
    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
        .set_revocation_url(revocation_url);

    // Set scopes (we want access to YouTube)
    let (authorize_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes([
            Scope::new("https://www.googleapis.com/auth/youtube.readonly".to_string()),
            Scope::new("https://www.googleapis.com/auth/youtube".to_string()),
            Scope::new("https://www.googleapis.com/auth/youtube.force-ssl".to_string()),
        ])
        .add_extra_param("access_type", "offline")
        .add_extra_param("prompt", "consent")
        .url();

    // Open browser for user login
    println!("Opening browser for auth...");
    open::that(authorize_url.as_str()).expect("Failed to open browser");

    let (code, state) = {
        // A very naive implementation of the redirect server.
        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

        // The server will terminate itself after collecting the first code.
        let Some(mut stream) = listener.incoming().flatten().next() else {
            panic!("listener terminated without accepting a connection");
        };

        let mut reader = BufReader::new(&stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line).unwrap();

        let redirect_url = request_line.split_whitespace().nth(1).unwrap();
        let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

        let code = url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
            .unwrap();

        let state = url
            .query_pairs()
            .find(|(key, _)| key == "state")
            .map(|(_, state)| CsrfToken::new(state.into_owned()))
            .unwrap();

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).unwrap();

        (code, state)
    };

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build client");

    // Exchange code for token
    let token_result = client
        .exchange_code(code)
        .request_async(&http_client).await
        .expect("Failed to request token");

    token_result
}

#[derive(Debug, serde::Deserialize)]
struct YoutubeApiError {
    error: YoutubeApiErrorDetails,
}

#[derive(Debug, serde::Deserialize)]
struct YoutubeApiErrorDetails {
    code: u16,
    message: String,
    // You can include more fields if needed
}

pub async fn get_active_livestream_chat_id(token: YoutubeToken) -> Result<Option<String>> {
    let client = Client::new();
    let response = client
        .get("https://www.googleapis.com/youtube/v3/liveBroadcasts")
        .bearer_auth(token.access_token().secret())
        .query(&[
            ("part", "id,snippet,status"),
            ("mine", "true"),
            ("maxResults", "1"),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_body = response.text().await.unwrap_or_default();

        if let Ok(parsed_error) = serde_json::from_str::<YoutubeApiError>(&error_body) {
            error!(
                "YouTube API error {}: {}",
                parsed_error.error.code, parsed_error.error.message
            );
        } else {
            error!(
                "Unexpected YouTube API error {} with raw body: {}",
                status, error_body
            );
        }

        return Ok(None);
    }

    info!("Youtube response: {:?}", response);
    let parsed = response.json::<LiveBroadcastListResponse>().await?;
    Ok(
        parsed.items
        .iter()
        .find(|b| b.status.lifeCycleStatus == "live")
        .and_then(|stream| stream.snippet.liveChatId.clone())
    )
}

pub async fn get_livestreams(token: YoutubeToken) -> Result<Vec<LiveBroadcast>, Box<dyn Error>> {
    let client = Client::new();
    let response_result = make_request(&client, &token).await?.error_for_status();
    let response = match response_result {
        Err(_) => {
            let new_token = get_new_youtube_token().await;
            make_request(&client, &new_token).await?.error_for_status()?
        }
        Ok(res) => res,
    };

    let parsed = response.json::<LiveBroadcastListResponse>().await?;
    Ok(parsed.items)
}

async fn make_request(client: &Client, token: &YoutubeToken) -> Result<Response, reqwest::Error> {
    let request = client
        .get("https://www.googleapis.com/youtube/v3/liveBroadcasts")
        .query(&[("mine", "true")])
        .bearer_auth(token.access_token().secret());
    println!("Request is {:?}", request);
    request.send().await
}

pub async fn fetch_live_chat_messages(
    token: YoutubeToken,
    live_chat_id: &str,
    page_token: Option<&str>,
) -> Result<LiveChatMessageListResponse> {
    let client = reqwest::Client::new();

    let mut url = format!(
        "https://www.googleapis.com/youtube/v3/liveChat/messages?part=snippet,authorDetails&liveChatId={}",
        live_chat_id
    );

    if let Some(token) = page_token {
        url.push_str(&format!("&pageToken={}", token));
    }

    let response = client
        .get(&url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await?
        .error_for_status()? // Converts non-2xx into an error
        .json::<LiveChatMessageListResponse>()
        .await?;

    Ok(response)
}

pub fn spawn_youtube_chat_stream(
    token: YoutubeToken,
    live_chat_id: String,
) -> tokio::sync::mpsc::Receiver<LiveChatMessage> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    task::spawn(async move {
        let mut next_page_token: Option<String> = None;
        let cutoff_time = Utc::now();

        loop {
            match fetch_live_chat_messages(token.clone(), &live_chat_id, next_page_token.as_deref()).await {
                Ok(response) => {
                    for item in response.items {
                        let published_at = &item.snippet.publishedAt;
                        if let Ok(published_time) = DateTime::parse_from_rfc3339(published_at) {
                            if published_time.with_timezone(&Utc) >= cutoff_time {
                                if tx.send(item).await.is_err() {
                                    break;
                                }
                            } else {
                                // Optionally log flushed message:
                                // println!("Flushed old message: {}", item.snippet.displayMessage);
                            }
                        } else {
                            // Discard if timestamp invalid
                            // println!("Invalid timestamp: {}", published_at);
                        }
                    }

                    next_page_token = response.nextPageToken;

                    //tokio::time::sleep(Duration::from_millis(
                    //    response.pollingIntervalMillis.unwrap_or(2000) + 500 as u64,
                    //))
                    //.await;
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
                Err(err) => {
                    eprintln!("Error fetching chat messages: {}", err);
                    break;
                }
            }
        }
    });

    rx
}
   
