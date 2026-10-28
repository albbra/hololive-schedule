use serde::Deserialize;
use slint::{Rgba8Pixel, SharedPixelBuffer};
use std::time::Duration;

use crate::translate::translate;
use crate::StreamData;

const API_URL: &str = "https://schedule.hololive.tv/api/list/7";

#[derive(Clone)]
pub struct StreamRow {
    pub date: String,
    pub show_date: bool,
    pub name: String,
    pub time: String,
    pub link: String,
    pub pic_url: String,
    pub title: String,
    pub is_live: bool,
}

impl StreamRow {
    pub fn to_ui(&self) -> StreamData {
        StreamData {
            date: self.date.clone().into(),
            show_date: self.show_date,
            name: self.name.clone().into(),
            time: self.time.clone().into(),
            link: self.link.clone().into(),
            title: self.title.clone().into(),
            is_live: self.is_live,
            pic: slint::Image::default(),
        }
    }
}

// ── API response types ──────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    date_group_list: Vec<DateGroup>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DateGroup {
    display_date: String,
    video_list: Vec<Video>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Video {
    display_date: String,
    is_live: bool,
    url: String,
    thumbnail: String,
    title: String,
    name: String,
}

// ── public API ──────────────────────────────────────────────────────────

pub fn fetch_streams() -> Result<Vec<StreamRow>, String> {
    static CLIENT: std::sync::OnceLock<reqwest::blocking::Client> =
        std::sync::OnceLock::new();
    let client = CLIENT.get_or_init(|| {
        reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (compatible; HololiveScheduleRust/0.1)")
            .timeout(Duration::from_secs(30))
            .build()
            .expect("client build")
    });

    let body = client
        .get(API_URL)
        .send().map_err(|e| format!("Request: {e}"))?
        .error_for_status().map_err(|e| format!("Server: {e}"))?
        .text().map_err(|e| format!("Read body: {e}"))?;

    let api: ApiResponse = serde_json::from_str(&body)
        .map_err(|e| format!("JSON parse: {e}"))?;

    let mut rows = Vec::new();
    let mut prev_date = String::new();

    for group in api.date_group_list {
        let date = group.display_date;
        for video in group.video_list {
            if video.name.is_empty() || video.url.is_empty() {
                continue;
            }

            let show_date = date != prev_date;
            if show_date {
                prev_date = date.clone();
            }

            rows.push(StreamRow {
                date: date.clone(),
                show_date,
                name: translate(&video.name),
                time: video.display_date,
                link: video.url,
                pic_url: video.thumbnail,
                title: video.title,
                is_live: video.is_live,
            });
        }
    }

    Ok(rows)
}

pub fn load_image(url: &str) -> Option<SharedPixelBuffer<Rgba8Pixel>> {
    if url.is_empty() {
        return None;
    }
    static CLIENT: std::sync::OnceLock<reqwest::blocking::Client> =
        std::sync::OnceLock::new();
    let client = CLIENT.get_or_init(|| {
        reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0")
            .timeout(Duration::from_secs(20))
            .build()
            .expect("image client")
    });
    let bytes = client.get(url).send().ok()?.bytes().ok()?;
    let img = image::load_from_memory(&bytes).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    Some(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(img.as_raw(), w, h))
}