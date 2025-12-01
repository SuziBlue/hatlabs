use crate::youtube::livestream::{LiveBroadcast, LiveBroadcastListResponse};
use crate::youtube::tokens::YoutubeToken;

use oauth2::TokenResponse;
use reqwest::{Client, Response};

pub async fn get_livestreams(token: YoutubeToken) -> Result<Vec<LiveBroadcast>, reqwest::Error> {
    let client = Client::new();
    let response = make_request(&client, &token).await?.error_for_status()?;

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
