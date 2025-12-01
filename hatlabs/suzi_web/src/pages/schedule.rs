use yew::prelude::*;
use wasm_bindgen_futures::{js_sys::Math::log, spawn_local};
use gloo_net::http::Request;
use gloo_console::log;
use serde::Deserialize;
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use std::collections::HashMap;

fn extract_date(published_at: &str) -> Option<NaiveDate> {
    match DateTime::parse_from_rfc3339(published_at) {
        Ok(dt) => Some(dt.naive_utc().date()),
        Err(e) => {
            log!(format!("Failed to parse date '{}': {:?}", published_at, e));
            None
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamInfo {
    pub title: String,
    pub scheduled_start: String,
    pub video_id: String,
    pub thumbnail_url: String,
}

#[function_component(Schedule)]
pub fn schedule() -> Html {
    // Map of all events fetched, grouped by date
    let all_events = use_state(HashMap::<NaiveDate, Vec<StreamInfo>>::new);

    // Track currently displayed month
    let today = Utc::now().naive_utc().date();
    let initial_month = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
    let displayed_month = use_state(|| initial_month);

    // Clone for fetch
    let all_events_clone = all_events.clone();

    // Fetch events only once
    use_effect_with((), move |_| {
        let all_events = all_events_clone;
        spawn_local(async move {
            let response = Request::get("/api/schedule")
                .send().await;

            if let Ok(response) = response {
                if let Ok(data) = response.json::<Vec<StreamInfo>>().await {
                    log!(format!("Fetched data: {:?}", data));
                    let mut map = HashMap::new();
                    for stream in data {
                        if let Some(date) = extract_date(&stream.scheduled_start) {
                            map.entry(date)
                                .or_insert_with(Vec::new)
                                .push(stream);
                        }
                    }
                    all_events.set(map);
                } else {
                    log!("Failed to parse JSON");
                }
            } else {
                log!("Failed to fetch data");
            }
        });

        || ()
    });

    // Navigation logic
    let on_prev = {
        let displayed_month = displayed_month.clone();
        Callback::from(move |_| {
            let new_month = *displayed_month - Duration::days(1); // Go to last day of previous month
            let prev_month = NaiveDate::from_ymd_opt(new_month.year(), new_month.month(), 1).unwrap();
            displayed_month.set(prev_month);
        })
    };

    let on_next = {
        let displayed_month = displayed_month.clone();
        Callback::from(move |_| {
            let next_month = if displayed_month.month() == 12 {
                NaiveDate::from_ymd_opt(displayed_month.year() + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(displayed_month.year(), displayed_month.month() + 1, 1).unwrap()
            };
            displayed_month.set(next_month);
        })
    };

    let (year, month) = (displayed_month.year(), displayed_month.month());
    let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let days_in_month = match month {
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    let mut calendar_cells = vec![];
    let starting_weekday = first_day.weekday().num_days_from_sunday();

    // Fill initial empty cells for alignment
    for _ in 0..starting_weekday {
        calendar_cells.push(html! { <div class="calendar-cell empty"></div> });
    }

    for day in 1..=days_in_month {
        let date = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let events_for_day = all_events.get(&date);

        calendar_cells.push(html! {
            <div class="calendar-cell">
            <strong>{ day }</strong>
                { for events_for_day.unwrap_or(&vec![]).iter().map(|stream| {
                    let video_url = format!("https://www.youtube.com/watch?v={}", stream.video_id);
                    html! {
                        <li class="event">
                            <a href={video_url} target="_blank">
                                <img src={stream.thumbnail_url.clone()} alt={stream.title.clone()} />
                                <div>{ &stream.title }</div>
                            </a>
                        </li>
                    }
                }) }
            </div>
        });
    }

    html! {
        <>
            <h1>{ "Livestream Calendar" }</h1>
            <div class="calendar-controls">
                <button onclick={on_prev}>{ "← Previous" }</button>
                <strong>{ format!("{} {}", first_day.format("%B"), year) }</strong>
                <button onclick={on_next}>{ "Next →" }</button>
            </div>
            <div class="calendar-grid">
                <div>{ "Sun" }</div>
                <div>{ "Mon" }</div>
                <div>{ "Tue" }</div>
                <div>{ "Wed" }</div>
                <div>{ "Thu" }</div>
                <div>{ "Fri" }</div>
                <div>{ "Sat" }</div>
                { for calendar_cells }
            </div>
        </>
    }
}
