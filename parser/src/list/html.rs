use std::str::FromStr;

use crate::Error;
use crate::ListItem;
use crate::Result;
use scraper::ElementRef;

struct HtmlParser {
    url: String,
    a_selector: scraper::Selector,
}

impl HtmlParser {
    fn parse_category(&self, element: &scraper::ElementRef) -> Result<String> {
        let a = element
            .select(&self.a_selector)
            .next()
            .ok_or_else(|| Error::HtmlMissingElement("a".into()))?;

        let category = a
            .value()
            .attr("href")
            .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?;

        let category = category
            .split('=')
            .nth(1)
            .ok_or_else(|| Error::ParseString(category.to_string()))?;

        Ok(category.to_string())
    }

    fn parse_title(&self, element: &scraper::ElementRef) -> Result<(String, String, usize)> {
        let mut a_it = element.select(&self.a_selector);

        let a0 = a_it
            .next()
            .ok_or_else(|| Error::HtmlMissingElement("a".into()))?;
        let a1 = a_it.next();

        let (title_a, comment_a) = if let Some(a) = a1 {
            (a, Some(a0))
        } else {
            (a0, None)
        };

        let title = title_a
            .value()
            .attr("title")
            .ok_or_else(|| Error::HtmlMissingAttribute("title".into()))?
            .to_string();

        let url = title_a
            .value()
            .attr("href")
            .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?
            .to_string();

        let comments = if let Some(comment_a) = comment_a {
            let comment = comment_a
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string();

            comment.parse::<usize>()?
        } else {
            0
        };

        Ok((title, url, comments))
    }

    fn parse_download(&self, element: &scraper::ElementRef) -> Result<(String, String)> {
        let mut a = element.select(&self.a_selector);

        let download = a
            .next()
            .ok_or_else(|| Error::HtmlMissingElement("a".into()))?
            .value()
            .attr("href")
            .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?
            .to_string();

        let magnet = a
            .next()
            .ok_or_else(|| Error::HtmlMissingElement("a".into()))?
            .value()
            .attr("href")
            .ok_or_else(|| Error::HtmlMissingAttribute("href".into()))?
            .to_string();

        Ok((download, magnet))
    }

    fn parse_size(&self, element: &scraper::ElementRef) -> Result<String> {
        let size = element
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        Ok(size)
    }

    fn parse_date(&self, element: &scraper::ElementRef) -> Result<chrono::DateTime<chrono::Utc>> {
        let date = element
            .value()
            .attr("data-timestamp")
            .ok_or_else(|| Error::HtmlMissingAttribute("data-timestamp".into()))?;

        let date = date
            .parse::<i64>()
            .map_err(|_| Error::ParseInteger(date.into()))?;

        let date = chrono::NaiveDateTime::from_timestamp_opt(date, 0)
            .ok_or_else(|| Error::ParseTimestamp(date.to_string()))?
            .and_local_timezone(chrono::Utc)
            .single()
            .ok_or_else(|| Error::ParseTimestamp(date.to_string()))?;

        Ok(date.with_timezone(&chrono::Utc))
    }

    fn parse_integer<T>(&self, element: &scraper::ElementRef) -> Result<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        let integer = element
            .text()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .to_string();

        integer
            .parse::<T>()
            .map_err(|e| Error::ParseInteger(e.to_string()))
    }

    fn parse_tr(&self, element: scraper::ElementRef) -> Result<ListItem> {
        let td_selector = scraper::Selector::parse("td").unwrap();
        let td_list = element.select(&td_selector).collect::<Vec<_>>();

        let category = td_list
            .get(0)
            .ok_or_else(|| Error::HtmlMissingElement("td (category)".into()))?;
        let title = td_list
            .get(1)
            .ok_or_else(|| Error::HtmlMissingElement("td (title)".into()))?;
        let download = td_list
            .get(2)
            .ok_or_else(|| Error::HtmlMissingElement("td (download)".into()))?;
        let size = td_list
            .get(3)
            .ok_or_else(|| Error::HtmlMissingElement("td (size)".into()))?;
        let date = td_list
            .get(4)
            .ok_or_else(|| Error::HtmlMissingElement("td (date)".into()))?;
        let seeders = td_list
            .get(5)
            .ok_or_else(|| Error::HtmlMissingElement("td (seeders)".into()))?;
        let leechers = td_list
            .get(6)
            .ok_or_else(|| Error::HtmlMissingElement("td (leechers)".into()))?;
        let downloads = td_list
            .get(7)
            .ok_or_else(|| Error::HtmlMissingElement("td (downloads)".into()))?;

        let category = self.parse_category(category)?;
        let (title, url, comments) = self.parse_title(title)?;
        let (download, magnet) = self.parse_download(download)?;
        let size = self.parse_size(size)?;
        let date = self.parse_date(date)?;
        let seeders = self.parse_integer(seeders)?;
        let leechers = self.parse_integer(leechers)?;
        let downloads = self.parse_integer(downloads)?;

        let trusted = element
            .value()
            .attr("class")
            .map(|s| s.contains("success"))
            .unwrap_or(false);

        let remake = element
            .value()
            .attr("class")
            .map(|s| s.contains("danger"))
            .unwrap_or(false);

        let url = format!("{}{}", self.url.trim_end_matches("/"), url);
        let download = format!("{}{}", self.url.trim_end_matches("/"), download);
        let id = url
            .split('/')
            .last()
            .ok_or_else(|| Error::HtmlMissingElement("id".into()))?
            .to_string();
        let id = id
            .parse::<usize>()
            .map_err(|_| Error::ParseInteger(id.clone()))?;

        let size = crate::parse_size(&size)?;
        let item = ListItem {
            guid: url,
            id,
            title,
            link: download.clone(),
            pub_date: date,
            seeders,
            leechers,
            downloads,
            category,
            size,
            comments,
            trusted,
            remake,
            download_link: Some(download),
            magnet_link: Some(magnet),
            description: None,
            info_hash: None,
        };

        Ok(item)
    }
}

pub fn parse(url: &str, data: &str) -> Result<Vec<ListItem>> {
    let parser = HtmlParser {
        url: url.to_string(),
        a_selector: scraper::Selector::parse("a").unwrap(),
    };

    let document = scraper::Html::parse_document(data);
    let selector = scraper::Selector::parse(".table > tbody:nth-child(2)").unwrap();
    let mut items = Vec::new();

    for element in document.select(&selector) {
        for child in element.children() {
            if let Some(element) = ElementRef::wrap(child) {
                if element.value().name() == "tr" {
                    items.push(parser.parse_tr(element)?);
                }
            }
        }
    }

    Ok(items)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let html = r#"
  <div class="table-responsive">
	<table class="table table-bordered table-hover table-striped torrent-list">
		<thead>
			<tr>
				<th class="hdr-category text-center" style="width:80px;">Category</th>
				<th class="hdr-name" style="width:auto;">Name</th>
				<th class="hdr-comments sorting text-center" title="Comments" style="width:50px;"><a href="/?s=comments&amp;o=desc"></a><i class="fa fa-comments-o"></i></th>
				<th class="hdr-link text-center" style="width:70px;">Link</th>
				<th class="hdr-size sorting text-center" style="width:100px;"><a href="/?s=size&amp;o=desc"></a>Size</th>
				<th class="hdr-date sorting_desc text-center" title="In UTC" style="width:140px;"><a href="/?s=id&amp;o=asc"></a>Date</th>

				<th class="hdr-seeders sorting text-center" title="Seeders" style="width:50px;"><a href="/?s=seeders&amp;o=desc"></a><i class="fa fa-arrow-up" aria-hidden="true"></i></th>
				<th class="hdr-leechers sorting text-center" title="Leechers" style="width:50px;"><a href="/?s=leechers&amp;o=desc"></a><i class="fa fa-arrow-down" aria-hidden="true"></i></th>
				<th class="hdr-downloads sorting text-center" title="Completed downloads" style="width:50px;"><a href="/?s=downloads&amp;o=desc"></a><i class="fa fa-check" aria-hidden="true"></i></th>
			</tr>
		</thead>
		<tbody>
			<tr class="danger">
				<td>
					<a href="/?c=1_3" title="Anime - Non-English-translated">
						<img src="/static/img/icons/nyaa/1_3.png" alt="Anime - Non-English-translated" class="category-icon">
					</a>
				</td>
				<td colspan="2">
					<a href="/view/1953481" title="[SweetSub][刹那之花][Momentary Lily][12][WebRip][1080P][AVC 8bit][简日内嵌]">[SweetSub][刹那之花][Momentary Lily][12][WebRip][1080P][AVC 8bit][简日内嵌]</a>
				</td>
				<td class="text-center">
					<a href="/download/1953481.torrent"><i class="fa fa-fw fa-download"></i></a>
					<a href="magnet:?xt=urn:btih:84e064742ffe9f5eb4a739766a33d8631746310c&amp;dn=%5BSweetSub%5D%5B%E5%88%B9%E9%82%A3%E4%B9%8B%E8%8A%B1%5D%5BMomentary%20Lily%5D%5B12%5D%5BWebRip%5D%5B1080P%5D%5BAVC%208bit%5D%5B%E7%AE%80%E6%97%A5%E5%86%85%E5%B5%8C%5D&amp;tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&amp;tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&amp;tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&amp;tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce"><i class="fa fa-fw fa-magnet"></i></a>
				</td>
				<td class="text-center">1.0 GiB</td>
				<td class="text-center" data-timestamp="1743239642">2025-03-29 09:14</td>

				<td class="text-center">5</td>
				<td class="text-center">41</td>
				<td class="text-center">1</td>
			</tr>
        </tbody>
    </table>
</div>
        "#;

        let results = super::parse("https://nyaa.si", html).unwrap();
        assert_eq!(results.len(), 1);

        let item = &results[0];
        assert_eq!(
            item.title,
            "[SweetSub][刹那之花][Momentary Lily][12][WebRip][1080P][AVC 8bit][简日内嵌]"
        );
        assert_eq!(item.link, "https://nyaa.si/download/1953481.torrent");
        assert_eq!(
            item.pub_date,
            chrono::DateTime::from_timestamp(1743239642, 0)
                .unwrap()
                .with_timezone(&chrono::Utc)
        );
        assert_eq!(item.guid, "https://nyaa.si/view/1953481");
        assert_eq!(item.id, 1953481);
        assert_eq!(item.seeders, 5);
        assert_eq!(item.leechers, 41);
        assert_eq!(item.downloads, 1);
        assert_eq!(item.category, "1_3");
        assert_eq!(item.size, 1073741824);
        assert_eq!(item.comments, 0);
        assert_eq!(item.trusted, false);
        assert_eq!(item.remake, true);
        assert_eq!(item.description, None);
        assert_eq!(
            item.download_link,
            Some("https://nyaa.si/download/1953481.torrent".to_string())
        );
        assert_eq!(item.magnet_link, Some("magnet:?xt=urn:btih:84e064742ffe9f5eb4a739766a33d8631746310c&dn=%5BSweetSub%5D%5B%E5%88%B9%E9%82%A3%E4%B9%8B%E8%8A%B1%5D%5BMomentary%20Lily%5D%5B12%5D%5BWebRip%5D%5B1080P%5D%5BAVC%208bit%5D%5B%E7%AE%80%E6%97%A5%E5%86%85%E5%B5%8C%5D&tr=http%3A%2F%2Fnyaa.tracker.wf%3A7777%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Fexodus.desync.com%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce".to_string()));
        assert_eq!(item.info_hash, None);
    }
}
