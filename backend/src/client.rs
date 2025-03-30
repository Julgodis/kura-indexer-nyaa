use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use reqwest::{Client as ReqwestClient, ClientBuilder, StatusCode, Url};
use tokio::sync::Mutex;

use crate::{data, html, rss};


#[derive(Debug, Clone)]
pub struct Client {
    inner: ReqwestClient,
    /// Minimum duration between two requests.
    min_interval: Duration,
    /// Protects the timestamp of the last request.
    last_request: Arc<Mutex<Instant>>,
    db_path: PathBuf
}

impl Client {
    /// Creates a new Client.
    ///
    /// * `requests_per_second` - Maximum number of requests allowed per second.
    /// * `event_tracker` - Optional callback that receives an Event after each request.
    pub fn new(min_interval: Duration, db_path: PathBuf) -> Result<Self> {
        let inner = ClientBuilder::new()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                         AppleWebKit/537.36 (KHTML, like Gecko) \
                         Chrome/58.0.3029.110 Safari/537.3")
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()?;
        Ok(Self {
            inner,
            // Calculate the minimum interval between requests.
            min_interval,
            // Initialize with a time far enough in the past so that the first request is not delayed.
            last_request: Arc::new(Mutex::new(Instant::now() - Duration::from_secs(1) - min_interval)),
            db_path,
        })
    }

    async fn  event_success(&self, url: Url, rate_limited: bool, event_type: &str) {
        let _ = self.event(url, rate_limited, event_type, "ok").await;
    }

    async fn event_failure(&self, url: Url, rate_limited: bool, event_type: &str) {
        let _ = self.event(url, rate_limited, event_type, "error").await;
    }

    async fn event(&self, url: Url, rate_limited: bool, event_type: &str, status: &str) -> anyhow::Result<()> {
        tracing::debug!(
            "Event: {} | URL: {} | Rate limited: {} | Status: {}",
            event_type,
            url,
            rate_limited,
            status
        );

        let db = rusqlite::Connection::open(&self.db_path)?;
        db.execute(
            "INSERT INTO events (url, rate_limited, event, status) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![url.as_str(), rate_limited, event_type, status],
        )?;

        Ok(())
    }

    /// Fetches data from the given URL, enforcing rate limiting and invoking the event tracker.
    pub async fn fetch_list(&self, url: Url) -> Result<Vec<data::Item>> {
        // Enforce the rate limit.
        let mut rate_limited = false;
        {
            let mut last = self.last_request.lock().await;
            let elapsed = last.elapsed();
            if elapsed < self.min_interval {
                tracing::debug!("Rate limited: {}ms", elapsed.as_millis());
                tokio::time::sleep(self.min_interval - elapsed).await;
                rate_limited = true;
            }
            *last = Instant::now();
        }

        // Fetch the data.
        let result = self.fetch_list_inner(url.clone()).await;
        match result {
            Ok(data) => {
                self.event_success(url.clone(), rate_limited, "list").await;
                Ok(data)
            }
            Err(err) => {
                tracing::error!("Failed to fetch data from {}: {:?}", url, err);
                self.event_failure(url.clone(), rate_limited, "list").await;
                Err(err)
            }
        }

    }

    /// Fetches data from the given URL, enforcing rate limiting and invoking the event tracker.
    async fn fetch_list_inner(&self, url: Url) -> Result<Vec<data::Item>> {
        let start = Instant::now();
        let response = self.inner
            .get(url.clone())
            .header("Accept", "application/xml, text/html, */*; q=0.9")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(anyhow::anyhow!("failed to fetch URL: {}", status));
        }

        let content_type = response
            .headers()
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .context("failed to get content type")?
            .to_string();
        tracing::trace!("content type: {}", content_type);

        let data_text = response.text().await?;
        tracing::trace!("fetched data: {}", data_text);

        let result = if content_type.contains("application/xml") {
            tracing::trace!("XML content type detected");
            rss::parse(&data_text)
        } else if content_type.contains("text/html") {
            tracing::trace!("HTML content type detected");
            html::parse_list(&data_text)
        } else {
            return Err(anyhow::anyhow!("unsupported content type: {}", content_type));
        };

        result
    }

    /// Fetches a view from the given URL.
    pub async fn fetch_view(&self, url: Url) -> Result<data::View> {
        // Enforce the rate limit.
        let mut rate_limited = false;
        {
            let mut last = self.last_request.lock().await;
            let elapsed = last.elapsed();
            if elapsed < self.min_interval {
                tokio::time::sleep(self.min_interval - elapsed).await;
                rate_limited = true;
            }
            *last = Instant::now();
        }

        // Fetch the data.
        let result = self.fetch_view_inner(url.clone()).await;
        match result {
            Ok(data) => {
                self.event_success(url.clone(), rate_limited, "view").await;
                Ok(data)
            }
            Err(err) => {
                tracing::error!("Failed to fetch data from {}: {:?}", url, err);
                self.event_failure(url.clone(), rate_limited, "view").await;
                Err(err)
            }
        }
    }

    async fn fetch_view_inner(&self, url: Url) -> anyhow::Result<data::View> {
        let response = self.inner
            .get(url)
            .header("Accept", "text/html, */*; q=0.9")
            .send()
            .await?;
    
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "failed to fetch URL: {}",
                response.status()
            ));
        }
    
        let data = response.text().await?;
        tracing::trace!("fetched data: {}", data);
        html::parse_view(&data)
    }
    
}
