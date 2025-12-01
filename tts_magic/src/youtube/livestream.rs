#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LiveBroadcastListResponse {
    pub kind: String,
    pub etag: String,
    pub nextPageToken: Option<String>,
    pub pageInfo: PageInfo,
    pub items: Vec<LiveBroadcast>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PageInfo {
    pub totalResults: u32,
    pub resultsPerPage: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LiveBroadcast {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Snippet,
    pub status: Status,
    pub contentDetails: Option<ContentDetails>,
    pub statistics: Option<Statistics>,
    pub monetizationDetails: Option<MonetizationDetails>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Snippet {
    pub publishedAt: String,
    pub channelId: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub thumbnails: Thumbnails,
    pub scheduledStartTime: Option<String>,
    pub actualStartTime: Option<String>,
    pub actualEndTime: Option<String>,
    pub isDefaultBroadcast: bool,
    pub liveChatId: Option<String>,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Thumbnails {
    #[serde(rename = "default")]
    pub default_thumb: Option<Thumbnail>,
    pub medium: Option<Thumbnail>,
    pub high: Option<Thumbnail>,
    pub standard: Option<Thumbnail>,
    pub maxres: Option<Thumbnail>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Status {
    pub lifeCycleStatus: String,
    pub privacyStatus: String,
    pub recordingStatus: String,
    pub madeForKids: bool,
    pub selfDeclaredMadeForKids: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ContentDetails {
    pub boundStreamId: Option<String>,
    pub boundStreamLastUpdateTimeMs: Option<String>,
    pub monitorStream: Option<MonitorStream>,
    pub enableEmbed: Option<bool>,
    pub enableDvr: Option<bool>,
    pub enableContentEncryption: Option<bool>,
    pub recordFromStart: Option<bool>,
    pub enableClosedCaptions: Option<bool>,
    pub closedCaptionsType: Option<String>,
    pub enableLowLatency: Option<bool>,
    pub latencyPreference: Option<String>,
    pub projection: Option<String>,
    pub enableAutoStart: Option<bool>,
    pub enableAutoStop: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MonitorStream {
    pub enableMonitorStream: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Statistics {
    pub concurrentViewers: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MonetizationDetails {
    pub cuepointSchedule: Option<CuepointSchedule>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CuepointSchedule {
    pub enabled: bool,
}
