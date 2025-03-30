use std::str::FromStr;

use kura_indexer::api;
use serde::{Deserialize, Serialize};

use crate::{data, html::parse_human_size};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rss {
    #[serde(rename = "channel")]
    channel: Channel,
}

impl Rss {
    pub fn to_items(&self) -> anyhow::Result<Vec<data::Item>> {
        self.channel.to_items()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    title: String,
    description: String,
    #[serde(rename = "item")]
    items: Vec<Item>,
}

impl Channel {
    pub fn to_items(&self) -> anyhow::Result<Vec<data::Item>> {
        self.items.iter().map(|item| item.to_item()).collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    title: String,
    link: String,
    guid: String,
    #[serde(rename = "pubDate")]
    pub_date: String,
    #[serde(rename = "seeders")]
    seeders: usize,
    #[serde(rename = "leechers")]
    leechers: usize,
    #[serde(rename = "downloads")]
    downloads: usize,
    #[serde(rename = "infoHash")]
    info_hash: String,
    #[serde(rename = "categoryId")]
    category_id: String,
    #[serde(rename = "category")]
    category: String,
    #[serde(rename = "size")]
    size: String,
    #[serde(rename = "comments")]
    comments: usize,
    #[serde(rename = "trusted")]
    trusted: String,
    #[serde(rename = "remake")]
    remake: String,
    description: String,
}

fn parse_boolean(value: &str) -> anyhow::Result<bool> {
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
        _ => Err(anyhow::anyhow!("invalid boolean value")),
    }
}

impl Item {
    pub fn to_item(&self) -> anyhow::Result<data::Item> {
        let date =
            chrono::DateTime::parse_from_rfc2822(&self.pub_date)?.with_timezone(&chrono::Utc);
        let category = data::Category::from_str(&self.category_id)
            .map_err(|_| anyhow::anyhow!("invalid category"))?;

        let trusted = parse_boolean(&self.trusted)?;
        let remake = parse_boolean(&self.remake)?;
        let size = parse_human_size(&self.size)?;

        Ok(data::Item {
            id: self.guid.clone(),
            title: self.title.clone(),
            link: self.link.clone(),
            date,
            seeders: self.seeders,
            leechers: self.leechers,
            downloads: self.downloads,
            category,
            size,
            comments: self.comments,
            trusted,
            remake,
            download_link: Some(self.link.clone()),
            magnet_link: None,
        })
    }
}

pub fn parse(data: &str) -> anyhow::Result<Vec<data::Item>> {
    let rss: Rss = serde_xml_rs::from_str(data)?;
    let items = rss
        .channel
        .to_items()
        .map_err(|e| anyhow::anyhow!("failed to parse recent items: {}", e))?;
    Ok(items)
}
