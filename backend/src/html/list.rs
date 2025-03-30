use std::str::FromStr;

use anyhow::Context;
use scraper::ElementRef;

use crate::data;

use super::parse_human_size;

fn parse_category(element: &scraper::ElementRef) -> anyhow::Result<data::Category> {
    let a = element
        .select(&scraper::Selector::parse("a").unwrap())
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing <a> element"))?;

    let category = a
        .value()
        .attr("href")
        .ok_or_else(|| anyhow::anyhow!("missing href attribute"))?;

    let category = category
        .split('=')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("missing category"))?;

    Ok(data::Category::from_str(category)?)
}

fn parse_title(element: &scraper::ElementRef) -> anyhow::Result<(String, String, usize)> {
    let a = element
        .select(&scraper::Selector::parse("a").unwrap())
        .collect::<Vec<_>>();

    let (title_url, comment) = if a.len() == 2 {
        (a[1], Some(a[0]))
    } else {
        (a[0], None)
    };

    let title = title_url
        .value()
        .attr("title")
        .ok_or_else(|| anyhow::anyhow!("missing title attribute"))?
        .to_string();

    let url = title_url
        .value()
        .attr("href")
        .ok_or_else(|| anyhow::anyhow!("missing href attribute"))?
        .to_string();

    let comments = if let Some(comment) = comment {
        let comment = comment
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        comment
            .parse::<usize>()
            .map_err(|_| anyhow::anyhow!("invalid comment"))?
    } else {
        0
    };

    Ok((title, url, comments))
}

fn parse_download(element: &scraper::ElementRef) -> anyhow::Result<(String, String)> {
    let binding = scraper::Selector::parse("a").unwrap();
    let mut a = element.select(&binding);

    let download = a
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing <a> element"))?
        .value()
        .attr("href")
        .ok_or_else(|| anyhow::anyhow!("missing href attribute"))?
        .to_string();

    let magnet = a
        .next()
        .ok_or_else(|| anyhow::anyhow!("missing <a> element"))?
        .value()
        .attr("href")
        .ok_or_else(|| anyhow::anyhow!("missing href attribute"))?
        .to_string();

    Ok((download, magnet))
}

fn parse_size(element: &scraper::ElementRef) -> anyhow::Result<String> {
    let size = element
        .text()
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string();

    Ok(size)
}

fn parse_date(element: &scraper::ElementRef) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
    let date = element
        .value()
        .attr("data-timestamp")
        .ok_or_else(|| anyhow::anyhow!("missing data-timestamp attribute"))?
        .parse::<i64>()
        .map_err(|_| anyhow::anyhow!("invalid timestamp"))?;

    let date = chrono::NaiveDateTime::from_timestamp_opt(date, 0)
        .ok_or_else(|| anyhow::anyhow!("invalid timestamp"))?
        .and_local_timezone(chrono::Utc)
        .single()
        .ok_or_else(|| anyhow::anyhow!("invalid timestamp"))?;
    let date = date.with_timezone(&chrono::Utc);
    Ok(date)
}

fn parse_integer<T>(element: &scraper::ElementRef) -> anyhow::Result<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let integer = element
        .text()
        .collect::<Vec<_>>()
        .join("")
        .trim()
        .to_string();

    let integer = integer
        .parse::<T>()
        .map_err(|_| anyhow::anyhow!("invalid integer"))?;
    Ok(integer)
}

fn parse_tr(element: scraper::ElementRef) -> anyhow::Result<data::Item> {
    let td_list = element
        .select(&scraper::Selector::parse("td").unwrap())
        .collect::<Vec<_>>();

    if td_list.len() != 8 {
        tracing::warn!("unexpected number of <td> elements: {}", td_list.len());
        return Err(anyhow::anyhow!("unexpected number of <td> elements"));
    }

    let category = parse_category(&td_list[0]).context("failed to parse category")?;
    let (title, url, comments) = parse_title(&td_list[1]).context("failed to parse title")?;
    let (download, magnet) = parse_download(&td_list[2]).context("failed to parse download")?;
    let size = parse_size(&td_list[3]).context("failed to parse size")?;
    let date = parse_date(&td_list[4]).context("failed to parse date")?;
    let seeders = parse_integer(&td_list[5]).context("failed to parse seeders")?;
    let leechers = parse_integer(&td_list[6]).context("failed to parse leechers")?;
    let downloads = parse_integer(&td_list[7]).context("failed to parse downloads")?;

    let trusted = element
        .value()
        .attr("class")
        .map(|s| s.contains("success"))
        .unwrap_or(false);

    let size = parse_human_size(&size)?;
    let item = data::Item {
        id: url.clone(),
        title,
        link: url,
        date,
        seeders,
        leechers,
        downloads,
        category,
        size,
        comments,
        trusted,
        remake: false,
        download_link: Some(download),
        magnet_link: Some(magnet),
    };

    Ok(item)
}

pub fn parse(data: &str) -> anyhow::Result<Vec<data::Item>> {
    let document = scraper::Html::parse_document(data);
    let selector = scraper::Selector::parse(".table > tbody:nth-child(2)").unwrap();
    let mut items = Vec::new();

    for element in document.select(&selector) {
        for child in element.children() {
            if let Some(element) = ElementRef::wrap(child) {
                if element.value().name() == "tr" {
                    items.push(parse_tr(element)?);
                }
            }
        }
    }

    Ok(items)
}
