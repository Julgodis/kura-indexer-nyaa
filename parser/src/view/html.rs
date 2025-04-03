use std::str::FromStr;

use chrono::TimeZone;
use scraper::{Html, Selector};

use crate::Category;
use crate::Error;
use crate::ListItem;
use crate::Result;
use crate::View;
use crate::ViewComment;
use crate::ViewFile;
use crate::parse_size;

pub fn parse(url: &str, data: &str) -> Result<View> {
    let data = data.replace("\t", " ").replace("\n", " ");

    let document = Html::parse_document(&data);

    let download_selector = Selector::parse("a[href^='/download/']").unwrap();
    let id = document
        .select(&download_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?;

    let id = id
        .split('/')
        .nth(2)
        .ok_or_else(|| {
            Error::ParseString(format!("Failed to parse ID from download link: {}", id))
        })?
        .replace(".torrent", "");
    let id = id.parse::<usize>()
        .map_err(|_| Error::ParseInteger(id.clone()))?;

    let title_selector = Selector::parse(".panel-title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .and_then(|el| el.text().next())
        .ok_or_else(|| Error::HtmlMissingElement(".panel-title".into()))?
        .trim()
        .to_string();

    let date_selector = Selector::parse("[data-timestamp]").unwrap();
    let timestamp = document
        .select(&date_selector)
        .next()
        .and_then(|el| el.value().attr("data-timestamp"))
        .ok_or_else(|| Error::HtmlMissingAttribute("data-timestamp".into()))?
        .parse::<i64>()?;

    let date = chrono::Utc.timestamp_opt(timestamp, 0).unwrap();

    let seeders = parse_number(&document, "Seeders:")?;
    let leechers = parse_number(&document, "Leechers:")?;
    let downloads = parse_number(&document, "Completed:")?;

    let submitter = parse_string(&document, "Submitter:")?;
    let info_link = parse_string(&document, "Information:").ok();
    let info_hash = parse_string(&document, "Info Hash:")?;

    let category_selector = Selector::parse(".col-md-5 a[href^='/?c=']").unwrap();
    let category_value = document
        .select(&category_selector)
        .last()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?;

    let category_value = category_value.split('=').nth(1).ok_or_else(|| {
        Error::ParseString(format!(
            "Failed to parse category value: {}",
            category_value
        ))
    })?;

    let category = Category::from_str(category_value)?;

    let file_size = parse_file_size(&document)?;

    let magnet_selector = Selector::parse("a[href^='magnet:']").unwrap();
    let magnet_link = document
        .select(&magnet_selector)
        .next()
        .and_then(|el| el.value().attr("href"))
        .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?
        .to_string();

    let description_selector = Selector::parse("#torrent-description").unwrap();
    let description = document
        .select(&description_selector)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or_default();

    let files = parse_files(&document)?;

    let comments = parse_comments(&document)?;

    let panel_danger_selector = Selector::parse(".panel-danger").unwrap();
    let panel_success_selector = Selector::parse(".panel-success").unwrap();

    let trusted = document
        .select(&panel_success_selector)
        .next()
        .iter()
        .count()
        > 0;
    let remake = document
        .select(&panel_danger_selector)
        .next()
        .iter()
        .count()
        > 0;

    let guid = format!("{}/view/{}", url.trim_end_matches("/"), id);
    let download_link = format!("{}/download/{}.torrent", url.trim_end_matches("/"), id);

    Ok(View {
        id,
        guid: guid.clone(),
        title,
        link: guid,
        pub_date: date,
        seeders,
        leechers,
        downloads,
        category,
        size: file_size,
        trusted,
        remake,
        magnet_link: Some(magnet_link),
        download_link: Some(download_link),
        description_md: description,
        files,
        comments,
        submitter,
        info_hash,
    })
}

fn find_value(document: &Html, label: &str) -> Result<String> {
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
                let next_child = children.next();
                if let Some(next) = next_child {
                    let number_text = next.text().collect::<String>();
                    return Ok(number_text.trim().to_string());
                }
                return Err(Error::HtmlMissingElement(format!(
                    "No number found after label: {}",
                    label
                )));
            }
        }
    }
    Err(Error::HtmlMissingElement(format!(
        "Label not found: {}",
        label
    )))
}

fn parse_number(document: &Html, label: &str) -> Result<usize> {
    find_value(document, label).and_then(|value| {
        value
            .parse::<usize>()
            .map_err(|_| Error::ParseInteger(value.clone()))
    })
}

fn parse_string(document: &Html, label: &str) -> Result<String> {
    find_value(document, label)
}

fn parse_file_size(document: &Html) -> Result<u64> {
    find_value(document, "File size:").and_then(|value| parse_size(&value))
}

fn parse_files(document: &Html) -> Result<Vec<ViewFile>> {
    let mut files = Vec::new();
    let file_selector = Selector::parse(".torrent-file-list li:not(:has(ul))").unwrap();

    for (index, file_element) in document.select(&file_selector).enumerate() {
        let file_text = file_element.text().collect::<String>();
        let parts: Vec<&str> = file_text.split('(').collect();

        if parts.len() >= 2 {
            let name = parts[0].trim().to_string();
            let size_part = parts.last().unwrap().trim();
            let size_text = size_part.trim_end_matches(')');

            let size = parse_size(size_text)?;

            files.push(ViewFile {
                id: index,
                name,
                size,
            });
        }
    }

    Ok(files)
}

fn parse_comments(document: &Html) -> Result<Vec<ViewComment>> {
    let mut comments = Vec::new();
    let comment_selector = Selector::parse(".comment-panel").unwrap();

    for comment_element in document.select(&comment_selector) {
        let id = comment_element
            .value()
            .attr("id")
            .unwrap_or("unknown")
            .replace("com-", "")
            .to_string();
        let id = id
            .parse::<usize>()
            .map_err(|_| Error::ParseInteger(id.clone()))?;

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
            .ok_or_else(|| Error::ParseString("Failed to parse date".into()))?;

        let edited_date = if edited > 0 {
            Some(
                chrono::Utc
                    .timestamp_opt(edited, 0)
                    .earliest()
                    .ok_or_else(|| Error::ParseString("Failed to parse edited date".into()))?,
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

        comments.push(ViewComment {
            id,
            user,
            date,
            edited_date: edited_date,
            content,
            avatar,
        });
    }

    Ok(comments)
}
