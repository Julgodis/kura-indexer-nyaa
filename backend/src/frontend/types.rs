use serde::{Deserialize, Serialize};

use crate::data;

#[derive(Deserialize, Serialize, Debug)]
pub struct TorrentListRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<data::Category>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<data::Filter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<data::Sort>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<data::SortOrder>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TorrentListResponse {
    pub torrents: Vec<Torrent>,
    pub offset: usize,
    pub count: usize,
    pub total: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Torrent {
    pub title: String,
    pub link: String,
    pub guid: String,
    pub pub_date: String,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub info_hash: String,
    pub category_id: String,
    pub category: String,
    pub size: u64,
    pub comments: usize,
    pub trusted: bool,
    pub remake: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Torrent {
    pub fn from(item: data::Item) -> Self {
        Self {
            title: item.title,
            link: item.link,
            guid: item.id,
            pub_date: item.date.to_rfc2822(),
            seeders: item.seeders,
            leechers: item.leechers,
            downloads: item.downloads,
            info_hash: "".to_string(), // Placeholder, as info_hash is not in Item
            category_id: item.category.to_string(),
            category: item.category.english(),
            size: item.size,
            comments: item.comments,
            trusted: item.trusted,
            remake: item.remake,
            description: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TorrentRequest {
    pub id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TorrentResponse {
    pub title: String,
    pub link: String,
    pub guid: String,
    pub pub_date: String,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub info_hash: String,
    pub category_id: String,
    pub category: String,
    pub size: u64,
    pub trusted: bool,
    pub remake: bool,
    pub description: String,
    pub description_markdown: String,
    pub download_link: String,
    pub magnet_link: String,
    pub files: Vec<data::File>,
    pub comments: Vec<data::Comment>,
    pub submitter: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_link: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TorrentsPerDayItem {
    pub date: String,
    pub count: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Event {
    RateLimit {
        url: String,
        elapsed: f64,
        min_interval: f64,
    },
    FetchList {
        url: String,
        error: Option<String>,
        elapsed: f64,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        item_count: Option<usize>,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        cached: Option<bool>,
    },
    FetchView {
        url: String,
        error: Option<String>,
        elapsed: f64,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        cached: Option<bool>,
    },
    Download {
        url: String,
        error: Option<String>,
        elapsed: f64,
        #[serde(default)]
        #[serde(skip_serializing_if = "Option::is_none")]
        cached: Option<bool>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EventItem {
    pub date: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub event_data: Event,
}
