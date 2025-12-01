use oauth2::basic::{BasicClient, BasicTokenType};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    ExtraTokenFields, RedirectUrl, RefreshToken, RevocationUrl, Scope, StandardTokenResponse,
    TokenType, TokenUrl,
};
use oauth2::{TokenResponse, reqwest};
use open;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::{Duration, SystemTime};
use std::{env, fs};
use url::Url;

pub mod api;
pub mod livestream;

pub type YoutubeToken = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Serialize, Deserialize)]
struct YoutubeTokenJar {
    youtube_token: YoutubeToken,
    expiration_time: SystemTime,
}

pub fn get_youtube_token() -> YoutubeToken {
    if let Ok(token_jar) = load_token() {
        println!("Token loaded, checking expiration time.");
        if token_jar.expiration_time > SystemTime::now() {
            println!("Token is still valid.");
            return token_jar.youtube_token;
        }
        println!("Token is expired.");
        println!("Attempting to refresh token.");
        return refresh_youtube_token(
            token_jar
                .youtube_token
                .refresh_token()
                .expect("Malformed token"),
        );
    }
    println!("Failed to load token, requesting new token.");
    let token = get_new_youtube_token();
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

fn refresh_youtube_token(refresh_token: &RefreshToken) -> YoutubeToken {
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

    // Set up the client
    let client = BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url);

    let http_client = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build client");

    let token = client
        .exchange_refresh_token(&refresh_token)
        .request(&http_client)
        .expect("Failed to exchange refresh token");

    token
}

fn get_new_youtube_token() -> YoutubeToken {
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

    let http_client = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build client");

    // Exchange code for token
    let token_result = client
        .exchange_code(code)
        .request(&http_client)
        .expect("Failed to request token");

    token_result
}
