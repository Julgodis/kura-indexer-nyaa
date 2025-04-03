use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    #[serde(rename = "0_0")]
    All,
    #[serde(rename = "1_0")]
    Anime,
    #[serde(rename = "1_1")]
    AnimeAmv,
    #[serde(rename = "1_2")]
    AnimeEnglish,
    #[serde(rename = "1_3")]
    AnimeNonEnglish,
    #[serde(rename = "1_4")]
    AnimeRaw,
    #[serde(rename = "2_0")]
    Audio,
    #[serde(rename = "2_1")]
    AudioLossless,
    #[serde(rename = "2_2")]
    AudioLossy,
    #[serde(rename = "3_0")]
    Literature,
    #[serde(rename = "3_1")]
    LiteratureEnglish,
    #[serde(rename = "3_2")]
    LiteratureNonEnglish,
    #[serde(rename = "3_3")]
    LiteratureRaw,
    #[serde(rename = "4_0")]
    LiveAction,
    #[serde(rename = "4_1")]
    LiveActionEnglish,
    #[serde(rename = "4_2")]
    LiveActionIdol,
    #[serde(rename = "4_3")]
    LiveActionNonEnglish,
    #[serde(rename = "4_4")]
    LiveActionRaw,
    #[serde(rename = "5_0")]
    Pictures,
    #[serde(rename = "5_1")]
    PicturesGraphics,
    #[serde(rename = "5_2")]
    PicturesPhotos,
    #[serde(rename = "6_0")]
    Software,
    #[serde(rename = "6_1")]
    SoftwareApps,
    #[serde(rename = "6_2")]
    SoftwareGames,
}

impl FromStr for Category {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "0_0" => Ok(Category::All),
            "1_0" => Ok(Category::Anime),
            "1_1" => Ok(Category::AnimeAmv),
            "1_2" => Ok(Category::AnimeEnglish),
            "1_3" => Ok(Category::AnimeNonEnglish),
            "1_4" => Ok(Category::AnimeRaw),
            "2_0" => Ok(Category::Audio),
            "2_1" => Ok(Category::AudioLossless),
            "2_2" => Ok(Category::AudioLossy),
            "3_0" => Ok(Category::Literature),
            "3_1" => Ok(Category::LiteratureEnglish),
            "3_2" => Ok(Category::LiteratureNonEnglish),
            "3_3" => Ok(Category::LiteratureRaw),
            "4_0" => Ok(Category::LiveAction),
            "4_1" => Ok(Category::LiveActionEnglish),
            "4_2" => Ok(Category::LiveActionIdol),
            "4_3" => Ok(Category::LiveActionNonEnglish),
            "4_4" => Ok(Category::LiveActionRaw),
            "5_0" => Ok(Category::Pictures),
            "5_1" => Ok(Category::PicturesGraphics),
            "5_2" => Ok(Category::PicturesPhotos),
            "6_0" => Ok(Category::Software),
            "6_1" => Ok(Category::SoftwareApps),
            "6_2" => Ok(Category::SoftwareGames),
            _ => Err(Error::ParseCategory(s.to_string())),
        }
    }
}

impl Category {
    pub fn as_id(&self) -> String {
        match self {
            Category::All => "0_0".to_string(),
            Category::Anime => "1_0".to_string(),
            Category::AnimeAmv => "1_1".to_string(),
            Category::AnimeEnglish => "1_2".to_string(),
            Category::AnimeNonEnglish => "1_3".to_string(),
            Category::AnimeRaw => "1_4".to_string(),
            Category::Audio => "2_0".to_string(),
            Category::AudioLossless => "2_1".to_string(),
            Category::AudioLossy => "2_2".to_string(),
            Category::Literature => "3_0".to_string(),
            Category::LiteratureEnglish => "3_1".to_string(),
            Category::LiteratureNonEnglish => "3_2".to_string(),
            Category::LiteratureRaw => "3_3".to_string(),
            Category::LiveAction => "4_0".to_string(),
            Category::LiveActionEnglish => "4_1".to_string(),
            Category::LiveActionIdol => "4_2".to_string(),
            Category::LiveActionNonEnglish => "4_3".to_string(),
            Category::LiveActionRaw => "4_4".to_string(),
            Category::Pictures => "5_0".to_string(),
            Category::PicturesGraphics => "5_1".to_string(),
            Category::PicturesPhotos => "5_2".to_string(),
            Category::Software => "6_0".to_string(),
            Category::SoftwareApps => "6_1".to_string(),
            Category::SoftwareGames => "6_2".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub title: String,
    pub link: String,
    pub pub_date: chrono::DateTime<chrono::Utc>,
    pub guid: String,
    pub id: usize,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub info_hash: Option<String>,
    pub category: Category,
    pub size: u64,
    pub comments: usize,
    pub trusted: bool,
    pub remake: bool,
    pub description: Option<String>,
    pub download_link: Option<String>,
    pub magnet_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewFile {
    pub id: usize,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewComment {
    pub id: usize,
    pub user: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub edited_date: Option<chrono::DateTime<chrono::Utc>>,
    pub content: String,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    pub title: String,
    pub link: String,
    pub pub_date: chrono::DateTime<chrono::Utc>,
    pub guid: String,
    pub id: usize,
    pub seeders: usize,
    pub leechers: usize,
    pub downloads: usize,
    pub info_hash: String,
    pub category: Category,
    pub size: u64,
    pub trusted: bool,
    pub remake: bool,
    pub description_md: String,
    pub download_link: Option<String>,
    pub magnet_link: Option<String>,
    pub files: Vec<ViewFile>,
    pub comments: Vec<ViewComment>,
    pub submitter: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse number: {0}")]
    ParseNumber(#[from] std::num::ParseIntError),
    #[error("Failed to parse date: {0}")]
    ParseDate(#[from] chrono::ParseError),


    #[error("Failed to parse XML: {0}")]
    ParseXml(#[from] serde_xml_rs::Error),

    #[error("HTML missing element: {0}")]
    HtmlMissingElement(String),
    #[error("HTML missing attribute: {0}")]
    HtmlMissingAttribute(String),
    #[error("HTML unexpected element: {0}")]
    HtmlUnexpectedElement(String),

    #[error("Unable to parse string: {0:?}")]
    ParseString(String),
    #[error("Unable to parse integer: {0:?}")]
    ParseInteger(String),
    #[error("Failed to parse boolean: {0:?}")]
    ParseBoolean(String),
    #[error("Failed to parse size: {1:?} ({0})")]
    ParseSize(String, #[source] Option<std::num::ParseFloatError>),
    #[error("Failed to parse category: {0:?}")]
    ParseCategory(String),
    #[error("Failed to timestamp: {0:?}")]
    ParseTimestamp(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod list;
pub mod view;

fn parse_boolean(value: &str) -> Result<bool> {
    match value {
        "0" => Ok(false),
        "1" => Ok(true),
        "None" => Ok(false),
        "True" => Ok(true),
        "False" => Ok(false),
        "true" => Ok(true),
        "false" => Ok(false),
        "yes" => Ok(true),
        "no" => Ok(false),
        "Yes" => Ok(true),
        "No" => Ok(false),
        _ => Err(crate::Error::ParseBoolean(value.to_string())),
    }
}

fn parse_size(value: &str) -> Result<u64> {
    let value = value.trim();
    let (num_str, unit) = if let Some(x) = value.strip_suffix(" B") {
        (x, 1u64)
    } else if let Some(x) = value.strip_suffix(" KB") {
        (x, 1000u64)
    } else if let Some(x) = value.strip_suffix(" MB") {
        (x, 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" GB") {
        (x, 1000u64 * 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" TB") {
        (x, 1000u64 * 1000u64 * 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" PB") {
        (x, 1000u64 * 1000u64 * 1000u64 * 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" KiB") {
        (x, 1024u64)
    } else if let Some(x) = value.strip_suffix(" MiB") {
        (x, 1024u64 * 1024u64)
    } else if let Some(x) = value.strip_suffix(" GiB") {
        (x, 1024u64 * 1024u64 * 1024u64)
    } else if let Some(x) = value.strip_suffix(" TiB") {
        (x, 1024u64 * 1024u64 * 1024u64 * 1024u64)
    } else if let Some(x) = value.strip_suffix(" PiB") {
        (x, 1024u64 * 1024u64 * 1024u64 * 1024u64 * 1024u64)
    } else {
        return Err(Error::ParseSize(value.to_string(), None));
    };

    let num = num_str.trim().replace(",", "");
    let num = num
        .parse::<f64>()
        .map_err(|e| Error::ParseSize(value.to_string(), Some(e)))?;
    let num = num * (unit as f64);
    let num = num.round() as u64;
    Ok(num)
}
