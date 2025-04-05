use std::path::PathBuf;

use reqwest::Url;

#[derive(Debug, Clone)]
pub struct RequestTracker {
    db_path: PathBuf,
}

impl RequestTracker {
    pub fn new(db_path: PathBuf) -> Self {
        let conn = rusqlite::Connection::open(db_path.clone()).expect("failed to open database");
        conn.execute(
            "CREATE TABLE IF NOT EXISTS requests (
                id INTEGER PRIMARY KEY,
                mirror_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                path TEXT NOT NULL,
                success INTEGER NOT NULL,
                cache_hit INTEGER NOT NULL,
                elapsed_time REAL NOT NULL
            )",
            [],
        )
        .expect("failed to create table");

        Self { db_path }
    }

    pub fn track_request_cached<Q>(&self, mirror_id: &str, url: &Url, query: &Q)
    where
        Q: serde::Serialize,
    {
        let mut url = url.clone();
        match serde_urlencoded::to_string(query) {
            Ok(query_string) => url.set_query(Some(&query_string)),
            Err(_) => url.set_query(None),
        }
        let full_path = url.as_str();
        self.register(mirror_id, &full_path, true, true, 0.0);
    }

    pub fn track_request<Q>(
        &self,
        mirror_id: &str,
        url: &Url,
        query: &Q,
        success: bool,
        elapsed_time: f64,
    ) where
        Q: serde::Serialize,
    {
        let mut url = url.clone();
        match serde_urlencoded::to_string(query) {
            Ok(query_string) => url.set_query(Some(&query_string)),
            Err(_) => url.set_query(None),
        }
        let full_path = url.as_str();
        self.register(mirror_id, &full_path, success, false, elapsed_time);
    }

    fn register(
        &self,
        mirror_id: &str,
        path: &str,
        success: bool,
        cache_hit: bool,
        elapsed_time: f64,
    ) {
        tracing::trace!(
            "registering request: mirror_id={}, path={}, success={}, cache_hit={}, elapsed_time={}",
            mirror_id,
            path,
            success,
            cache_hit,
            elapsed_time
        );
        let conn =
            rusqlite::Connection::open(self.db_path.clone()).expect("failed to open database");
        match conn.execute(
            "INSERT INTO requests (mirror_id, timestamp, path, success, cache_hit, elapsed_time) VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                mirror_id,
                chrono::Utc::now().to_rfc3339(),
                path,
                success as i32,
                cache_hit as i32,
                elapsed_time
            ],
        )
        {
            Ok(_) => {}
            Err(e) => {
                tracing::warn!("failed to insert request into database: {}", e);
            }
        }
    }

    pub fn get_requests(
        &self,
        mirror_id: &str,
    ) -> Vec<(chrono::DateTime<chrono::Utc>, String, bool, bool, f64)> {
        let conn =
            rusqlite::Connection::open(self.db_path.clone()).expect("failed to open database");
        let mut stmt = conn
            .prepare("SELECT timestamp, path, success, cache_hit, elapsed_time FROM requests WHERE mirror_id = ? ORDER BY timestamp DESC LIMIT 250")
            .expect("failed to prepare statement");
        let rows = stmt.query_map([mirror_id], |row| {
            Ok((
                row.get::<_, chrono::DateTime<chrono::Utc>>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
                row.get::<_, bool>(3)?,
                row.get::<_, f64>(4)?,
            ))
        });
        let rows = match rows {
            Ok(rows) => rows,
            Err(e) => {
                tracing::warn!("failed to query requests: {}", e);
                return vec![];
            }
        };

        match rows.collect::<Result<Vec<_>, _>>() {
            Ok(requests) => requests,
            Err(e) => {
                tracing::warn!("failed to collect requests: {}", e);
                vec![]
            }
        }
    }
}
