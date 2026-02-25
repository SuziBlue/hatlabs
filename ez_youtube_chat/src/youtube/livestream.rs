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
    pub contentDetails: ContentDetails,
    pub statistics: Option<Statistics>,
    pub monetizationDetails: Option<MonetizationDetails>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Snippet {
    publishedAt: String,
    channelId: String,
    title: String,
    description: String,
    thumbnails: Thumbnails,
    scheduledStartTime: Option<String>,
    actualStartTime: Option<String>,
    actualEndTime: Option<String>,
    isDefaultBroadcast: bool,
    liveChatId: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Thumbnails {
    #[serde(rename = "default")]
    default_thumb: Option<Thumbnail>,
    medium: Option<Thumbnail>,
    high: Option<Thumbnail>,
    standard: Option<Thumbnail>,
    maxres: Option<Thumbnail>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
struct Thumbnail {
    url: String,
    width: u32,
    height: u32,
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
    pub monitorStream: MonitorStream,
    pub enableEmbed: bool,
    pub enableDvr: bool,
    pub enableContentEncryption: bool,
    pub recordFromStart: bool,
    pub enableClosedCaptions: bool,
    pub closedCaptionsType: String,
    pub enableLowLatency: bool,
    pub latencyPreference: String,
    pub projection: String,
    pub enableAutoStart: bool,
    pub enableAutoStop: bool,
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
    pub cuepointSchedule: CuepointSchedule,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CuepointSchedule {
    pub enabled: bool,
}
