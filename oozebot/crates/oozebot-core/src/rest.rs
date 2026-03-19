use std::marker::PhantomData;

use either::Either;
use oozebot_protocol::resources::guild::Snowflake;
use reqwest::{Body, Method, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::Value;





pub struct DiscordRequest<Obj> {
    method: Method,
    url: String,
    auth: Authorization,
    user_agent: UserAgent,
    _marker: PhantomData<Obj>
}

pub struct DiscordResponse<Obj> {
    object: Obj,
    _marker: PhantomData<Obj>
}

#[derive(Debug, Deserialize)]
pub struct DiscordError {
    code: i32,
    errors: Value,
    message: String,
}

pub enum Authorization {
    Bot(String),
    Bearer(String)
}

pub struct UserAgent {
    url: String,
    version_number: String
}

pub struct Channels;

pub struct Messages;

pub struct Guilds;

pub struct Users;

pub struct Permissions;


pub struct EndpointBuilder<End> {
    _marker: PhantomData<End>,
    endpoint: String
}

macro_rules! endpoint {
    ($name:ident($id:ident), $segment:literal, $state:ty) => {
        fn $name(self, $id: Snowflake) -> EndpointBuilder<$state> {
            self.push($segment).push_id($id).cast()
        }
    };

    ($name:ident, $segment:literal, $state:ty) => {
        fn $name(self) -> EndpointBuilder<$state> {
            self.push($segment).cast()
        }
    };
}

impl<End> EndpointBuilder<End> {
    pub fn new() -> EndpointBuilder<()> {
        EndpointBuilder { _marker: PhantomData, endpoint: String::new() }
    }
    fn push(mut self, segment: &str) -> Self {
        self.endpoint.push('/');
        self.endpoint.push_str(segment);
        self
    }
    fn push_id(mut self, id: Snowflake) -> Self {
        self.endpoint.push('/');
        self.endpoint.push_str(&id.to_string());
        self
    }
    fn cast<NewEnd>(self) -> EndpointBuilder<NewEnd> {
        EndpointBuilder { _marker: PhantomData, endpoint: self.endpoint }
    }
}

impl EndpointBuilder<()> {
    endpoint!(users(id), "users", Users);
    endpoint!(me, "@me", Users);
    endpoint!(channels(id), "channels", Channels);
}

impl EndpointBuilder<Channels> {
    endpoint!(permissions(id), "permissions", Permissions);
}

trait Endpoint<Obj> {
    fn to_url(self) -> String;
}

impl<End, Obj> Endpoint<Obj> for EndpointBuilder<End> {
    fn to_url(self) -> String {
        self.endpoint
    }
}


pub fn make_request<Obj>(
    method: Method,
    auth: Authorization,
    user_agent: UserAgent,
    endpoint: impl Endpoint<Obj>,
) -> DiscordRequest<Obj> {
    let url = format!("https://discord.com/api/v10{}", endpoint.to_url());

    DiscordRequest {
        method,
        url,
        auth,
        user_agent,
        _marker: PhantomData
    }
}

pub async fn send_request<Obj: DeserializeOwned>(
    request: DiscordRequest<Obj>,
) -> Result<DiscordResponse<Obj>, Either<reqwest::Error, DiscordError>> {
    let client = reqwest::Client::new();

    let mut builder = client
        .request(request.method, &request.url)
        .header("User-Agent", request.user_agent.to_string())
        .header("Authorization", request.auth.to_string());

    let response = builder.send().await.map_err(Either::Left)?;

    let status = response.status();
    let body = response.text().await.map_err(Either::Left)?;

    if !status.is_success() {
        let err: DiscordError = response.json().await.map_err(Either::Left)?;
        return Err(Either::Right(err))
    }

    let object: Obj = response.json().await.map_err(Either::Left)?;

    Ok(DiscordResponse {
        object,
        _marker: PhantomData,
    })
}
