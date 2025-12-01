use crate::youtube::get_new_youtube_token;

use super::{
    YoutubeToken, get_youtube_token,
    livestream::{LiveBroadcast, LiveBroadcastListResponse},
};
use oauth2::TokenResponse;
use reqwest::blocking::{Client, Response};
use std::error::Error;

pub fn get_livestreams(token: YoutubeToken) -> Result<Vec<LiveBroadcast>, Box<dyn Error>> {
    let client = Client::new();
    let response_result = make_request(&client, &token)?.error_for_status();
    let response = match response_result {
        Err(_) => {
            let new_token = get_new_youtube_token();
            make_request(&client, &new_token)?.error_for_status()?
        }
        Ok(res) => res,
    };

    let parsed = response.json::<LiveBroadcastListResponse>()?;
    Ok(parsed.items)
}

fn make_request(client: &Client, token: &YoutubeToken) -> Result<Response, reqwest::Error> {
    let request = client
        .get("https://www.googleapis.com/youtube/v3/liveBroadcasts")
        .query(&[("mine", "true")])
        .bearer_auth(token.access_token().secret());
    println!("Request is {:?}", request);
    request.send()
}
