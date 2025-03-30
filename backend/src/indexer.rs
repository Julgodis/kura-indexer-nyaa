use std::{path::PathBuf, time::Duration};

use kura_indexer::{
    api::{
        self,
        search::{SearchRequest, SearchResponse},
        test::{TestRequest, TestResponse},
    },
    server::Indexer,
};

use crate::{client, data, frontend::types::{EventItem, TorrentsPerDayItem}};

#[derive(Debug, Clone)]
pub struct NyaaContext {
    pub update_interval: Duration,
    pub db_path: PathBuf,
    pub event_db_path: PathBuf,
    pub client: client::Client,
}

impl NyaaContext {
    pub fn add_items(&self, items: Vec<data::Item>) -> anyhow::Result<()> {
        let db = rusqlite::Connection::open(&self.db_path)?;

        let mut stmt = db.prepare(
            r#"
INSERT INTO items (
    id, title, link, date, 
    seeders, leechers, downloads, 
    category, size, comments, 
    trusted, remake,
    download_link, magnet_link
) VALUES (
    ?, ?, ?, ?,
    ?, ?, ?,
    ?, ?, ?,
    ?, ?,
    ?, ?
) ON CONFLICT(id) DO UPDATE SET
    title = excluded.title,
    link = excluded.link,
    date = excluded.date,
    seeders = excluded.seeders,
    leechers = excluded.leechers,
    downloads = excluded.downloads,
    category = excluded.category,
    size = excluded.size,
    comments = excluded.comments,
    trusted = excluded.trusted,
    remake = excluded.remake,
    download_link = excluded.download_link,
    magnet_link = excluded.magnet_link
    WHERE items.id = excluded.id;
"#,
        )?;

        for item in items {
            stmt.execute((
                item.id,
                item.title,
                item.link,
                item.date,
                item.seeders,
                item.leechers,
                item.downloads,
                item.category,
                item.size,
                item.comments,
                item.trusted,
                item.remake,
                item.download_link,
                item.magnet_link,
            ))?;
        }

        Ok(())
    }

    pub fn db(&self) -> anyhow::Result<rusqlite::Connection> {
        Ok(rusqlite::Connection::open(&self.db_path)?)
    }

    pub fn event_db(&self) -> anyhow::Result<rusqlite::Connection> {
        Ok(rusqlite::Connection::open(&self.event_db_path)?)
    }

    pub fn get_item_count(db: &rusqlite::Connection) -> anyhow::Result<usize> {
        let mut stmt = db.prepare("SELECT COUNT(*) FROM items;")?;
        let count: usize = stmt.query_row([], |row| row.get(0))?;

        Ok(count)
    }

    pub fn get_items(
        db: &rusqlite::Connection,
        offset: Option<usize>,
        count: Option<usize>,
        since: Option<i64>,
        category: Option<data::Category>,
        filter: Option<data::Filter>,
        sort: Option<data::Sort>,
        sort_order: Option<data::SortOrder>,
    ) -> anyhow::Result<(usize, usize, Vec<data::Item>)> {
        let offset = offset.unwrap_or(0);
        let count = count.unwrap_or(75);
        let count = if count < 1 { 1 } else { count };

        let offset = if offset % count != 0 {
            offset - (offset % count)
        } else {
            offset
        };

        let mut args = Vec::<&dyn rusqlite::ToSql>::new();
        let mut query = "SELECT * FROM items".to_string();

        let mut where_clauses = Vec::new();
        if let Some(_) = since {
            where_clauses.push("date > ?");
            args.push(&since);
        }
        if let Some(x) = category {
            match x {
                data::Category::All => {}
                _ => {
                    where_clauses.push("category = ?");
                    args.push(&category);
                }
            }
        }
        if let Some(filter) = filter {
            match filter {
                data::Filter::NoRemake => {
                    where_clauses.push("remake = 0");
                }
                data::Filter::Trusted => {
                    where_clauses.push("trusted = 1");
                }
                _ => {}
            }
        }

        if !where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&where_clauses.join(" AND "));
        }

        if let Some(sort) = sort {
            match sort {
                data::Sort::Date => query.push_str(" ORDER BY date"),
                data::Sort::Seeders => query.push_str(" ORDER BY seeders"),
                data::Sort::Leechers => query.push_str(" ORDER BY leechers"),
                data::Sort::Downloads => query.push_str(" ORDER BY downloads"),
                data::Sort::Size => query.push_str(" ORDER BY size"),
            }
        } else {
            query.push_str(" ORDER BY date");
        }
        if let Some(sort_order) = sort_order {
            match sort_order {
                data::SortOrder::Ascending => query.push_str(" ASC"),
                data::SortOrder::Descending => query.push_str(" DESC"),
            }
        } else {
            query.push_str(" DESC");
        }

        query.push_str(" LIMIT ? OFFSET ?");
        args.push(&count);
        args.push(&offset);

        tracing::debug!("Query:\n{}", query);

        let mut stmt = db.prepare(&query)?;

        let items = stmt.query_map(args.as_slice(), |row| {
            Ok(data::Item {
                id: row.get(0)?,
                title: row.get(1)?,
                link: row.get(2)?,
                date: row.get(3)?,
                seeders: row.get(4)?,
                leechers: row.get(5)?,
                downloads: row.get(6)?,
                category: row.get(7)?,
                size: row.get(8)?,
                comments: row.get(9)?,
                trusted: row.get(10)?,
                remake: row.get(11)?,
                download_link: row.get(12)?,
                magnet_link: row.get(13)?,
            })
        })?;
        let items = items.collect::<Result<Vec<_>, _>>()?;
        Ok((offset, count, items))
    }

    pub fn get_torrent_per_day(db: &rusqlite::Connection) -> anyhow::Result<Vec<TorrentsPerDayItem>> {
        let mut stmt = db.prepare(
            r#"
SELECT strftime('%Y-%m-%d', date) AS rounded_date, COUNT(*) AS count
FROM items
GROUP BY rounded_date
ORDER BY rounded_date DESC
LIMIT 30;
"#,
        )?;

        let items = stmt.query_map([], |row| {
            Ok(TorrentsPerDayItem {
                date: row.get::<_, String>(0)?,
                count: row.get::<_, usize>(1)?,
            })
        })?;
        let mut items = items.collect::<Result<Vec<_>, _>>()?;
        items.reverse();
        Ok(items)
    }

    pub fn get_events(
        db: &rusqlite::Connection,
    ) -> anyhow::Result<Vec<EventItem>> {
        let mut stmt = db.prepare(
            r#"
SELECT url, rate_limited, event, status, date
FROM events
ORDER BY date DESC
LIMIT 100;
"#,
        )?;

        let items = stmt.query_map([], |row| {
            Ok(EventItem {
                url: row.get(0)?,
                rate_limited: row.get(1)?,
                event: row.get(2)?,
                status: row.get(3)?,
                date: row.get(4)?,
            })
        })?;
        let items = items.collect::<Result<Vec<_>, _>>()?;
        Ok(items)
    }
}

#[derive(Debug, Clone)]
pub struct NyaaIndexer {}

#[async_trait::async_trait]
impl Indexer for NyaaIndexer {
    type Context = NyaaContext;

    async fn test(context: Self::Context, request: TestRequest) -> api::Result<TestResponse> {
        tracing::info!("Test request: {:?}", request);
        Ok(TestResponse {})
    }

    async fn search(context: Self::Context, request: SearchRequest) -> api::Result<SearchResponse> {
        tracing::info!("Search request: {:?}", request);
        Ok(SearchResponse {})
    }

    async fn recent(
        context: Self::Context,
        request: api::RecentRequest,
    ) -> api::Result<api::RecentResponse> {
        tracing::info!("Recent request: {:?}", request);

        let response: anyhow::Result<_> = (|| {
            let db = context.db()?;
            let item_count = NyaaContext::get_item_count(&db)?;
            let (offset, count, items) = NyaaContext::get_items(
                &db,
                request.offset,
                request.count,
                request.since.map(|s| s.timestamp()),
                None,
                None,
                None,
                None,
            )?;

            let items = items
                .into_iter()
                .map(|item| api::RecentItem {
                    id: item.id,
                    title: item.title,
                    date: item.date,
                    seeders: item.seeders,
                    leechers: item.leechers,
                    category: item.category.to_string(),
                    size: item.size,
                })
                .collect::<Vec<_>>();

            Ok(api::RecentResponse {
                interval: context.update_interval,
                offset,
                count,
                total: item_count,
                items,
            })
        })();

        match response {
            Ok(response) => Ok(response),
            Err(e) => {
                tracing::error!("Failed to get recent items: {:?}", e);
                Err(api::ApiError::InternalError)
            }
        }
    }
}
