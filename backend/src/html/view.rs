use std::str::FromStr;

use crate::data;
use anyhow::Context;
use chrono::TimeZone;
use scraper::{Html, Selector};

use super::parse_human_size;

pub fn parse(data: &str) -> anyhow::Result<data::View> {
    let data = data.replace("\t", " ").replace("\n", " ");

    let document = Html::parse_document(&data);

    // Find ID from /download/{}.torrent
    let download_selector = Selector::parse("a[href^='/download/']").unwrap();
    let id = document
        .select(&download_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find download link"))?
        .split('/')
        .nth(2)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse ID from download link"))?
        .replace(".torrent", "");
    let id = id.parse::<usize>()?;
    tracing::trace!("parsed ID: {}", id);

    // Parse title
    let title_selector = Selector::parse(".panel-title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .and_then(|el| el.text().next())
        .ok_or_else(|| anyhow::anyhow!("Failed to find title"))?
        .trim()
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

    let submitter = parse_string(&document, "Submitter:")?;
    let info_link = parse_string(&document, "Information:").ok();
    let info_hash = parse_string(&document, "Info Hash:")?;

    // Parse category
    let category_selector = Selector::parse(".col-md-5 a[href^='/?c=']").unwrap();
    let category_value = document
        .select(&category_selector)
        .last()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| anyhow::anyhow!("Failed to find category"))?
        .split('=')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse category value"))?;

    let category = data::Category::from_str(category_value)
        .context("Invalid category")?;

    // Parse file size
    let file_size = parse_file_size(&document)?;

    // Parse links
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

    Ok(data::View {
        id: format!("/view/{}", id),
        title,
        link: format!("/view/{}", id),
        date,
        seeders,
        leechers,
        downloads,
        category,
        size: file_size,
        trusted,
        remake,
        magnet_link,
        download_link: format!("/download/{}.torrent", id),
        description,
        files,
        comments,
        submitter,
        info_hash,
        info_link,
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
                tracing::trace!("found label: {:?}", text);
                let next_child = children.next();
                if let Some(next) = next_child {
                    let number_text = next.text().collect::<String>();
                    tracing::trace!("found value: {:?}", number_text);
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

fn parse_string(document: &Html, label: &str) -> anyhow::Result<String> {
    find_value(document, label).context(format!("Failed to parse string for label: {}", label))
}

fn parse_file_size(document: &Html) -> anyhow::Result<u64> {
    find_value(document, "File size:")
        .and_then(|value| parse_human_size(&value).context("Failed to parse file size"))
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

            let size = parse_human_size(size_text)
                .context(anyhow::anyhow!("Failed to parse file size"))?;

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
        let mut timestamp_it = comment_element.select(&timestamp_selector);
        let timestamp = timestamp_it
            .next()
            .and_then(|el| el.value().attr("data-timestamp"))
            .unwrap_or("0")
            .parse::<i64>()?;

        let edited = if let Some(edited_el) = timestamp_it.next() {
            edited_el
                .value()
                .attr("data-timestamp")
                .unwrap_or("0")
                .parse::<i64>()
                .unwrap_or(0)
        } else {
            0
        };

        let date = chrono::Utc
            .timestamp_opt(timestamp, 0)
            .earliest()
            .context("Failed to parse date")?;

        let edited_date = if edited > 0 {
            Some(
                chrono::Utc
                    .timestamp_opt(edited, 0)
                    .earliest()
                    .context("Failed to parse edited date")?,
            )
        } else {
            None
        };

        let content_selector = Selector::parse(".comment-content").unwrap();
        let content = comment_element
            .select(&content_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_default();

        let avatar_selector = Selector::parse(".avatar").unwrap();
        let avatar = comment_element
            .select(&avatar_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .unwrap_or("https://nyaa.si/static/img/avatars/default.png")
            .to_string();

        let avatar = if avatar.contains("default.png") {
            None
        } else {
            Some(avatar)
        };

        comments.push(data::Comment {
            id,
            user,
            date,
            edited: edited_date,
            content,
            avatar,
        });
    }

    Ok(comments)
}
