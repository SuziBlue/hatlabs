
use reqwest;
use serde::{Deserialize, Serialize};
use warp::reject::Rejection;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Deserialize, Clone)]
struct SearchResponse {
    items: Vec<SearchItem>,
}

#[derive(Debug, Deserialize, Clone)]
struct SearchItem {
    id: SearchId,
    snippet: Snippet,
}

#[derive(Debug, Deserialize, Clone)]
struct SearchId {
    videoId: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Snippet {
    title: String,
    thumbnails: Thumbnails,
}

#[derive(Debug, Deserialize, Clone)]
struct Thumbnails {
    #[serde(rename = "default")]
    default_thumb: Thumbnail,

    #[serde(rename = "medium")]
    medium_thumb: Option<Thumbnail>,

    #[serde(rename = "high")]
    high_thumb: Option<Thumbnail>,
}

#[derive(Debug, Deserialize, Clone)]
struct Thumbnail {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
struct VideoResponse {
    items: Vec<VideoItem>,
}

#[derive(Debug, Deserialize, Clone)]
struct VideoItem {
    id: String,
    liveStreamingDetails: LiveStreamingDetails,
}

#[derive(Debug, Deserialize, Clone)]
struct LiveStreamingDetails {
    #[serde(default)]
    scheduledStartTime: Option<String>,
    #[serde(default)]
    actualStartTime: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
struct StreamInfo {
    title: String,
    video_id: String,
    scheduled_start: String,
    thumbnail_url: String,
}

static SCHEDULE_CACHE: LazyLock<Arc<RwLock<Option<CachedSchedule>>>> = LazyLock::new(|| {
    Arc::new(RwLock::new(None))
});

struct CachedSchedule {
    timestamp: DateTime<Utc>,
    data: Vec<StreamInfo>,
}

pub async fn get_schedule(api_key: String, channel_id: String) -> Result<warp::reply::Json, Rejection> {
    let now = Utc::now();
    let cache = SCHEDULE_CACHE.clone();

    {
        let read_guard = cache.read().await;

        if let Some(cached) = &*read_guard {
            if now.signed_duration_since(cached.timestamp) < Duration::minutes(60) {
                // Return cached response
                return Ok(warp::reply::json(&cached.data));
            }
        }
    }

    // Proceed to fetch new data
    let mut all_items = Vec::new();
    let event_types = ["upcoming", "live", "completed"];

    for event_type in event_types.iter() {
        let search_url = format!(
            "https://www.googleapis.com/youtube/v3/search?part=snippet&channelId={}&eventType={}&type=video&maxResults=50&order=date&key={}",
            channel_id, event_type, api_key
        );

        let search_resp = reqwest::get(&search_url).await
            .map_err(|_| warp::reject::not_found())?;

        let search_json: SearchResponse = search_resp.json().await
            .map_err(|_| warp::reject::not_found())?;

        all_items.extend(search_json.items);
    }

    if all_items.is_empty() {
        return Ok(warp::reply::json(&Vec::<StreamInfo>::new()));
    }

    let video_ids: Vec<String> = all_items.iter().map(|item| item.id.videoId.clone()).collect();
    let video_ids_param = video_ids.join(",");

    let videos_url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=liveStreamingDetails&id={}&key={}",
        video_ids_param, api_key
    );

    let videos_resp = reqwest::get(&videos_url).await
        .map_err(|_| warp::reject::not_found())?;

    let videos_json: VideoResponse = videos_resp.json().await
        .map_err(|_| warp::reject::not_found())?;

    let mut video_map = std::collections::HashMap::new();
    for video in videos_json.items {
        let timestamp = video.liveStreamingDetails
            .scheduledStartTime
            .or(video.liveStreamingDetails.actualStartTime);

        if let Some(ts) = timestamp {
            video_map.insert(video.id.clone(), ts);
        }
    }

    let results: Vec<StreamInfo> = all_items.into_iter()
        .filter_map(|item| {
            let thumbnail_url = item.snippet
                .thumbnails
                .high_thumb
                .as_ref()
                .or(item.snippet.thumbnails.medium_thumb.as_ref())
                .map(|t| t.url.clone())
                .unwrap_or_else(|| item.snippet.thumbnails.default_thumb.url.clone());

            video_map.get(&item.id.videoId).map(|start| StreamInfo {
                title: item.snippet.title,
                video_id: item.id.videoId.clone(),
                scheduled_start: start.clone(),
                thumbnail_url,
            })
        })
        .collect();

    // Update cache
    {
        let mut write_guard = cache.write().await;
        *write_guard = Some(CachedSchedule {
            timestamp: now,
            data: results.clone(),
        });
    }

    Ok(warp::reply::json(&results))
}
