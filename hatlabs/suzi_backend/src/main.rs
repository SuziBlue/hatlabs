use warp::{Filter, http::{Method, HeaderValue}, Rejection, Reply};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

mod youtube_schedule;

use youtube_schedule::get_schedule;

pub fn domain() -> &'static str {
    option_env!("DOMAIN").unwrap_or("http://localhost:3030")
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BlogPost {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub content: String,
}

async fn get_all_posts() -> Result<impl Reply, Rejection> {
    let file_path = "cdn/blog.json";
    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => match serde_json::from_str::<Vec<BlogPost>>(&content) {
            Ok(posts) => Ok(warp::reply::json(&posts)),
            Err(_) => Err(warp::reject::not_found()),
        },
        Err(_) => Err(warp::reject::not_found()),
    }
}

async fn get_post_by_id(post_id: usize) -> Result<impl Reply, Rejection> {
    let file_path = "cdn/blog.json";
    match tokio::fs::read_to_string(file_path).await {
        Ok(content) => match serde_json::from_str::<Vec<BlogPost>>(&content) {
            Ok(posts) => {
                if let Some(post) = posts.into_iter().find(|p| p.id == post_id) {
                    Ok(warp::reply::json(&post))
                } else {
                    Err(warp::reject::not_found())
                }
            }
            Err(_) => Err(warp::reject::not_found()),
        },
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    
    // Load env vars
    let api_key = std::env::var("YOUTUBE_API_KEY").expect("Missing YOUTUBE_API_KEY");
    let channel_id = std::env::var("CHANNEL_ID").expect("Missing CHANNEL_ID");

    // === 1. API Routes ===
    let images_route = warp::path("images.json")
        .and(warp::get())
        .map(|| {
            let mut filenames = Vec::new();
            if let Ok(entries) = std::fs::read_dir("cdn") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            filenames.push(name.to_string());
                        }
                    }
                }
            }
            warp::reply::json(&filenames)
        });

    let blog_route = warp::path!("api" / "blog.json")
        .and(warp::get())
        .and_then(get_all_posts);

    let post_route = warp::path!("api" / "post" / usize)
        .and(warp::get())
        .and_then(get_post_by_id);

    let schedule_route = warp::path!("api" / "schedule")
        .and(warp::get())
        .and_then(move || {
            let api_key = api_key.clone();
            let channel_id = channel_id.clone();
            get_schedule(api_key, channel_id) // You can move the logic to a `get_schedule` fn
        });

    let api_routes = images_route
        .or(blog_route)
        .or(post_route)
        .or(schedule_route);

    // === 2. Static Frontend Files ===

    // Serve index.html at `/`
    let index_route = warp::path::end()
        .and(warp::fs::file("cdn/index.html"));

    // Serve all static files like .wasm, .js, etc.
    let static_files = warp::fs::dir("cdn");

    // Fallback route for SPA: serve index.html for unmatched frontend routes
    let fallback_route = warp::any()
        .and(warp::get())
        .and(warp::fs::file("cdn/index.html"));

    let cors = {
        let origin_str = domain();
        let origin_header = HeaderValue::from_str(origin_str)
            .expect("Invalid FRONTEND_ORIGIN for CORS");

        warp::cors()
            .allow_origin(origin_header)
            .allow_methods(&[Method::GET])
            .allow_headers(vec!["Content-Type"])
    };

    // === 3. Combine Routes ===
    let routes = api_routes
        .or(index_route)
        .or(static_files)
        .or(fallback_route)
        .with(warp::log("request_logger"))
        .with(cors);

    println!("📡 Serving at {}", domain());
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
