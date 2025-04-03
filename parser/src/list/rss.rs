use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Category, ListItem, Result, parse_boolean, parse_size};

#[derive(Debug, Deserialize, Serialize)]
struct Rss {
    #[serde(rename = "channel")]
    channel: Channel,
}

impl Rss {
    fn to_items(self) -> Vec<Item> {
        self.channel.to_items()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Channel {
    title: String,
    description: String,
    #[serde(rename = "item")]
    items: Vec<Item>,
}

impl Channel {
    fn to_items(self) -> Vec<Item> {
        self.items
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
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

fn to_item(item: Item) -> Result<ListItem> {
    let date = chrono::DateTime::parse_from_rfc2822(&item.pub_date)?.with_timezone(&chrono::Utc);
    let category = Category::from_str(&item.category_id)?;

    let trusted = parse_boolean(&item.trusted)?;
    let remake = parse_boolean(&item.remake)?;
    let size = parse_size(&item.size)?;

    let id = item
        .guid
        .split('/')
        .last()
        .ok_or_else(|| crate::Error::ParseString(item.guid.clone()))?
        .to_string();
    let id = id
        .parse::<usize>()
        .map_err(|_| crate::Error::ParseInteger(id.clone()))?;

    Ok(ListItem {
        guid: item.guid.clone(),
        id,
        title: item.title.clone(),
        link: item.link.clone(),
        pub_date: date,
        seeders: item.seeders,
        leechers: item.leechers,
        downloads: item.downloads,
        info_hash: Some(item.info_hash),
        category,
        size,
        comments: item.comments,
        trusted,
        remake,
        description: Some(item.description),
        download_link: Some(item.link),
        magnet_link: None,
    })
}

pub fn parse(data: impl AsRef<str>) -> Result<Vec<ListItem>> {
    let rss: Rss = serde_xml_rs::from_str(data.as_ref())?;
    let items = rss.to_items();
    Ok(items.into_iter().map(to_item).collect::<Result<Vec<_>>>()?)
}

#[cfg(test)]
mod tests {
    use crate::Category;

    #[test]
    fn test_parse() {
        let xml = r#"
<rss xmlns:atom="http://www.w3.org/2005/Atom" xmlns:nyaa="https://nyaa.si/xmlns/nyaa" version="2.0">
    <channel>
        <title>Nyaa - Home - Torrent File RSS</title>
        <description>RSS Feed for Home</description>
        <link>https://nyaa.si/</link>
        <atom:link href="https://nyaa.si/?page=rss" rel="self" type="application/rss+xml"/>
        <item>
            <title>[Sokudo] The Super Cube S01E03 [1080p AV1] (weekly)</title>
            <link>https://nyaa.si/download/1953465.torrent</link>
            <guid isPermaLink="true">https://nyaa.si/view/1953465</guid>
            <pubDate>Sat, 29 Mar 2025 06:51:19 -0000</pubDate>
            <nyaa:seeders>59</nyaa:seeders>
            <nyaa:leechers>12</nyaa:leechers>
            <nyaa:downloads>93</nyaa:downloads>
            <nyaa:infoHash>6a1093801c4567cf75ab148d4db88651ce3b25e3</nyaa:infoHash>
            <nyaa:categoryId>1_2</nyaa:categoryId>
            <nyaa:category>Anime - English-translated</nyaa:category>
            <nyaa:size>205.9 MiB</nyaa:size>
            <nyaa:comments>0</nyaa:comments>
            <nyaa:trusted>No</nyaa:trusted>
            <nyaa:remake>No</nyaa:remake>
            <description><![CDATA[<a href="https://nyaa.si/view/1953465">#1953465 | [Sokudo] The Super Cube S01E03 [1080p AV1] (weekly)</a> | 205.9 MiB | Anime - English-translated | 6A1093801C4567CF75AB148D4DB88651CE3B25E3]]></description>
        </item>
    </channel>
</rss>"#;

        let results = super::parse(xml).unwrap();
        assert_eq!(results.len(), 1);

        let item = &results[0];
        assert_eq!(
            item.title,
            "[Sokudo] The Super Cube S01E03 [1080p AV1] (weekly)"
        );
        assert_eq!(item.link, "https://nyaa.si/download/1953465.torrent");
        assert_eq!(
            item.pub_date,
            chrono::DateTime::parse_from_rfc2822("Sat, 29 Mar 2025 06:51:19 -0000")
                .unwrap()
                .with_timezone(&chrono::Utc)
        );
        assert_eq!(item.guid, "https://nyaa.si/view/1953465");
        assert_eq!(item.id, 1953465);
        assert_eq!(item.seeders, 59);
        assert_eq!(item.leechers, 12);
        assert_eq!(item.downloads, 93);
        assert_eq!(
            item.info_hash,
            Some("6a1093801c4567cf75ab148d4db88651ce3b25e3".to_string())
        );
        assert_eq!(item.category, Category::AnimeEnglish);
        assert_eq!(item.size, 215_901_798);
        assert_eq!(item.comments, 0);
        assert_eq!(item.trusted, false);
        assert_eq!(item.remake, false);
        assert_eq!(
            item.description,
            Some("<a href=\"https://nyaa.si/view/1953465\">#1953465 | [Sokudo] The Super Cube S01E03 [1080p AV1] (weekly)</a> | 205.9 MiB | Anime - English-translated | 6A1093801C4567CF75AB148D4DB88651CE3B25E3".to_string())
        );
        assert_eq!(
            item.download_link,
            Some("https://nyaa.si/download/1953465.torrent".to_string())
        );
        assert_eq!(item.magnet_link, None);
    }
}
