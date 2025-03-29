use std::str::FromStr;

use crate::{data, html::parse_size_string};
use anyhow::Context;
use chrono::TimeZone;
use scraper::{Html, Selector};

pub fn parse(data: &str) -> anyhow::Result<data::View> {
    let data = data.replace("\t", " ").replace("\n", " ");

    let document = Html::parse_document(&data);

    // Parse title
    let title_selector = Selector::parse(".panel-title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .and_then(|el| el.text().next())
        .ok_or_else(|| anyhow::anyhow!("Failed to find title"))?
        .trim()
        .to_string();

    // Parse ID from URL or download link
    let download_selector = Selector::parse("a[href^='/download/']").unwrap();
    let download_link = document
        .select(&download_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find download link"))?;

    let id = download_link
        .split('/')
        .last()
        .and_then(|s| s.split('.').next())
        .ok_or_else(|| anyhow::anyhow!("Failed to parse ID from download link"))?
        .to_string();

    // Parse date
    let date_selector = Selector::parse("[data-timestamp]").unwrap();
    let timestamp = document
        .select(&date_selector)
        .next()
        .and_then(|el| el.value().attr("data-timestamp"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find timestamp"))?
        .parse::<i64>()?;

    let date = chrono::Utc.timestamp_opt(timestamp, 0).unwrap();

    // Parse seeders, leechers, and downloads
    let seeders = parse_number(&document, "Seeders:")?;
    let leechers = parse_number(&document, "Leechers:")?;
    let downloads = parse_number(&document, "Completed:")?;

    // Parse category
    let category_selector = Selector::parse(".col-md-5 a[href^='/?c=']").unwrap();
    let category_value = document
        .select(&category_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find category"))?
        .split('=')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse category value"))?;

    let category = data::Category::from_str(category_value)
        .map_err(|_| anyhow::anyhow!("Invalid category"))?;

    // Parse file size
    let file_size = parse_file_size(&document)?;

    // Parse links
    let full_download_link = format!("https://nyaa.si{}", download_link);

    let magnet_selector = Selector::parse("a[href^='magnet:']").unwrap();
    let magnet_link = document
        .select(&magnet_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find magnet link"))?
        .to_string();

    // Parse description
    let description_selector = Selector::parse("#torrent-description").unwrap();
    let description = document
        .select(&description_selector)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or_default();

    // Parse files
    let files = parse_files(&document)?;

    // Parse comments
    let comments = parse_comments(&document)?;

    // Determine trusted and remake status (these might be visible in a different part of the page)
    let trusted = false; // Default value, would need to be extracted from the submitter info
    let remake = false; // Default value, would need to be extracted from specific markers

    // Construct the full URL (this is an assumption, modify as needed)
    let link = format!("https://nyaa.si/view/{}", id);

    Ok(data::View {
        id,
        title,
        link,
        date,
        seeders,
        leechers,
        downloads,
        category,
        size: file_size,
        trusted,
        remake,
        download_link: full_download_link,
        magnet_link,
        description,
        files,
        comments,
    })
}

fn find_value(document: &Html, label: &str) -> anyhow::Result<String> {
    let label = label.to_lowercase();
    let selector = Selector::parse(".row").unwrap();
    let child_selector = Selector::parse("div").unwrap();
    for row in document.select(&selector) {
        let mut children = row.select(&child_selector);
        while let Some(child) = children.next() {
            let text = child
                .text()
                .collect::<String>()
                .trim()
                .to_lowercase()
                .to_string();
            if text.contains(&label) {
                tracing::debug!("Found label: {:?}", text);
                let next_child = children.next();
                if let Some(next) = next_child {
                    let number_text = next.text().collect::<String>();
                    tracing::debug!("Found value: {:?}", number_text);
                    return Ok(number_text.trim().to_string());
                }
                return Err(anyhow::anyhow!("No number found after label: {}", label));
            }
        }
    }
    Err(anyhow::anyhow!("Label not found: {}", label))
}

fn parse_number(document: &Html, label: &str) -> anyhow::Result<usize> {
    find_value(document, label)
        .and_then(|value| value.parse::<usize>().context("Failed to parse number"))
        .context(format!("Failed to parse number for label: {}", label))
}

fn parse_file_size(document: &Html) -> anyhow::Result<u64> {
    find_value(document, "File size:")
        .and_then(|value| parse_size_string(&value).context("Failed to parse file size"))
        .context("Failed to parse file size")
}

fn parse_files(document: &Html) -> anyhow::Result<Vec<data::File>> {
    let mut files = Vec::new();
    let file_selector = Selector::parse(".torrent-file-list li:not(:has(ul))").unwrap();

    for (index, file_element) in document.select(&file_selector).enumerate() {
        let file_text = file_element.text().collect::<String>();
        let parts: Vec<&str> = file_text.split('(').collect();

        if parts.len() >= 2 {
            let name = parts[0].trim().to_string();
            let size_part = parts.last().unwrap().trim();
            let size_text = size_part.trim_end_matches(')');

            let size = parse_size_string(size_text)
                .map_err(|_| anyhow::anyhow!("Failed to parse file size"))?;

            files.push(data::File {
                id: index.to_string(), // Using index as ID since original IDs aren't provided
                name,
                size,
            });
        }
    }

    Ok(files)
}

fn parse_comments(document: &Html) -> anyhow::Result<Vec<data::Comment>> {
    let mut comments = Vec::new();
    let comment_selector = Selector::parse(".comment-panel").unwrap();

    for comment_element in document.select(&comment_selector) {
        let id = comment_element
            .value()
            .attr("id")
            .unwrap_or("unknown")
            .replace("com-", "")
            .to_string();

        let user_selector = Selector::parse(".col-md-2 a").unwrap();
        let user = comment_element
            .select(&user_selector)
            .next()
            .and_then(|el| el.text().next())
            .unwrap_or("Anonymous")
            .trim()
            .to_string();

        let timestamp_selector = Selector::parse("[data-timestamp]").unwrap();
        let timestamp = comment_element
            .select(&timestamp_selector)
            .next()
            .and_then(|el| el.value().attr("data-timestamp"))
            .unwrap_or("0")
            .parse::<i64>()?;

        let date = chrono::Utc.timestamp_opt(timestamp, 0).unwrap();

        let content_selector = Selector::parse(".comment-content").unwrap();
        let content = comment_element
            .select(&content_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_default();

        comments.push(data::Comment {
            id,
            user,
            date,
            content,
        });
    }

    Ok(comments)
}
