use oauth2::basic::{BasicClient, BasicTokenType, BasicErrorResponseType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields, EndpointNotSet, EndpointSet, RedirectUrl, RefreshToken, RevocationUrl, Scope, StandardTokenResponse, TokenUrl, StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse, RevocationErrorResponseType
};
use oauth2::{TokenResponse, reqwest};
use open;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::env::VarError;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::SystemTime;
use std::{env, fs};
use url::Url;

type Client = oauth2::Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, StandardRevocableToken, StandardErrorResponse<RevocationErrorResponseType>, EndpointSet, EndpointNotSet, EndpointNotSet, EndpointSet, EndpointSet>;
pub type YoutubeToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Debug, Serialize, Deserialize)]
struct YoutubeTokenJar {
    youtube_token: YoutubeToken,
    expiration_time: SystemTime,
}

pub struct AuthManager {
    client: Client,
    token: Option<YoutubeToken>,
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Environement variable {0} not found.")]
    EnvError(String),
    
}
impl From<VarError> for AuthError {
    fn from(value: VarError) -> Self {
        Self::EnvError(value.to_string())
    }
}

async fn client_from_env() -> Result<Client, AuthError> {
   let client_id = ClientId::new(
       env::var("YOUTUBE_CLIENT_ID")?
           .to_string(),
   );
   let client_secret = ClientSecret::new(
       env::var("YOUTUBE_CLIENT_SECRET")?
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
    Ok(client)
}

pub async fn get_youtube_token() -> Result<YoutubeToken, AuthError> {
    let client = client_from_env();
    if let Ok(token_jar) = load_token() {

        println!("Token loaded, checking expiration time.");
        if token_jar.expiration_time > SystemTime::now() {
            println!("Token is still valid.");
            return Ok(token_jar.youtube_token);
        }

        println!("Token is expired.");
        println!("Attempting to refresh token.");
        let token = refresh_youtube_token(
            &client.await?,
            token_jar
                .youtube_token
                .refresh_token()
                .expect("Malformed token"),
        ).await;

        return Ok(token);
    }

    println!("Failed to load token, requesting new token.");
    let token = get_new_youtube_token(&client.await?).await;
    let expiration_time = SystemTime::now() + token.expires_in().expect("Malformed token");
    let token_jar = YoutubeTokenJar {
        youtube_token: token.clone(),
        expiration_time: expiration_time,
    };
    save_token(&token_jar).expect("Failed to save token");

    Ok(token)
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

async fn refresh_youtube_token(client: &Client, refresh_token: &RefreshToken) -> YoutubeToken {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build client");

    let token = client
        .exchange_refresh_token(&refresh_token)
        .request_async(&http_client).await
        .expect("Failed to exchange refresh token");

    token
}

async fn get_new_youtube_token(client: &Client) -> YoutubeToken {
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
