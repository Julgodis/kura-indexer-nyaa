use serde::{Deserialize, Serialize};

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
    pub category: String,
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
    pub category: String,
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
    let value = value.trim().to_ascii_lowercase();
    if value.is_empty() {
        return Ok(0);
    }
    let (num_str, unit) = if let Some(x) = value.strip_suffix(" b") {
        (x, 1u64)
    } else if let Some(x) = value.strip_suffix(" bytes") {
        (x, 1u64)
    } else if let Some(x) = value.strip_suffix(" kn") {
        (x, 1000u64)
    } else if let Some(x) = value.strip_suffix(" mb") {
        (x, 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" gb") {
        (x, 1000u64 * 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" tb") {
        (x, 1000u64 * 1000u64 * 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" pb") {
        (x, 1000u64 * 1000u64 * 1000u64 * 1000u64 * 1000u64)
    } else if let Some(x) = value.strip_suffix(" kib") {
        (x, 1024u64)
    } else if let Some(x) = value.strip_suffix(" mib") {
        (x, 1024u64 * 1024u64)
    } else if let Some(x) = value.strip_suffix(" gib") {
        (x, 1024u64 * 1024u64 * 1024u64)
    } else if let Some(x) = value.strip_suffix(" tib") {
        (x, 1024u64 * 1024u64 * 1024u64 * 1024u64)
    } else if let Some(x) = value.strip_suffix(" pib") {
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
