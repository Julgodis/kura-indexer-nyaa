use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use reqwest::{Client as ReqwestClient, ClientBuilder, StatusCode, Url};
use tokio::sync::Mutex;

use crate::frontend::types::Event;
use crate::{data, html, rss};

#[derive(Debug, Clone)]
pub struct Client {
    inner: ReqwestClient,
    /// Minimum duration between two requests.
    min_interval: Duration,
    /// Protects the timestamp of the last request.
    last_request: Arc<Mutex<Instant>>,
    db_path: PathBuf,
    max_retries: usize,
}

impl Client {
    /// Creates a new Client.
    pub fn new(
        min_interval: Duration,
        db_path: PathBuf,
        connect_timeout: Duration,
        timeout: Duration,
        max_retries: usize,
    ) -> Result<Self> {
        let inner = ClientBuilder::new()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                         AppleWebKit/537.36 (KHTML, like Gecko) \
                         Chrome/58.0.3029.110 Safari/537.3",
            )
            .connect_timeout(connect_timeout)
            .timeout(timeout)
            .build()?;
        Ok(Self {
            inner,
            min_interval,
            last_request: Arc::new(Mutex::new(
                Instant::now() - Duration::from_secs(1) - min_interval,
            )),
            db_path,
            max_retries,
        })
    }

    fn event<T>(&self, date: chrono::DateTime<chrono::Utc>, event_type: &str, event_data: T)
    where
        T: serde::Serialize,
    {
        let result = (|| -> anyhow::Result<()> {
            let event_data = serde_json::to_string(&event_data)?;
            let db = rusqlite::Connection::open(&self.db_path)?;
            db.execute(
                "INSERT INTO events (date, event_type, event_data) VALUES (?1, ?2, ?3)",
                rusqlite::params![date, event_type, event_data],
            )?;

            Ok(())
        })();

        if let Err(err) = result {
            tracing::error!("failed to log event: {:?}", err);
        }
    }

    pub async fn fetch_list(&self, url: Url) -> Result<Vec<data::Item>> {
        self.rate_limit(&url).await;

        let timer = Instant::now();
        let result = self.fetch_list_inner(url.clone()).await;
        let elapsed = timer.elapsed();
        self.event(
            chrono::Utc::now(),
            "fetch_list",
            Event::FetchList {
                url: url.to_string(),
                error: result.as_ref().err().map(|e| e.to_string()),
                elapsed: elapsed.as_secs_f64(),
            },
        );
        match result {
            Ok(data) => Ok(data),
            Err(err) => {
                tracing::error!("failed to fetch data from {}: {:?}", url, err);
                Err(err)
            }
        }
    }

    async fn fetch_list_inner(&self, url: Url) -> Result<Vec<data::Item>> {
        let fetch = |url: Url| async move {
            let response = self
                .inner
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
                return Err(anyhow::anyhow!(
                    "unsupported content type: {}",
                    content_type
                ));
            };

            result
        };

        self.retry_fetch(url, fetch).await
    }

    async fn rate_limit(&self, url: &Url) {
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        if elapsed < self.min_interval {
            self.event(
                chrono::Utc::now(),
                "rate_limit",
                Event::RateLimit {
                    url: url.to_string(),
                    elapsed: elapsed.as_secs_f64(),
                    min_interval: self.min_interval.as_secs_f64(),
                },
            );
            tokio::time::sleep(self.min_interval - elapsed).await;
        }
        *last = Instant::now();
    }

    pub async fn fetch_view(&self, url: Url) -> Result<data::View> {
        self.rate_limit(&url).await;

        let timer = Instant::now();
        let result = self.fetch_view_inner(url.clone()).await;
        let elapsed = timer.elapsed();
        self.event(
            chrono::Utc::now(),
            "fetch_view",
            Event::FetchView {
                url: url.to_string(),
                error: result.as_ref().err().map(|e| e.to_string()),
                elapsed: elapsed.as_secs_f64(),
            },
        );
        match result {
            Ok(data) => Ok(data),
            Err(err) => {
                tracing::error!("failed to fetch data from {}: {:?}", url, err);
                Err(err)
            }
        }
    }

    async fn retry_fetch<F, P, R>(&self, url: Url, fetch: F) -> Result<R>
    where
        F: Fn(Url) -> P,
        P: std::future::Future<Output = Result<R>>,
    {
        let mut retries = 0;
        loop {
            match fetch(url.clone()).await {
                Ok(data) => return Ok(data),
                Err(err) => {
                    if retries >= self.max_retries {
                        return Err(err);
                    }
                    tracing::warn!("failed to fetch data: {:?}", err);
                    retries += 1;
                    tokio::time::sleep(Duration::from_secs(1) + self.min_interval).await;
                    tracing::warn!("retrying... (attempt {})", retries);
                }
            }
        }
    }

    async fn fetch_view_inner(&self, url: Url) -> anyhow::Result<data::View> {
        let fetch = |url: Url| async move {
            let response = self
                .inner
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
        };

        //self.retry_fetch(url, fetch).await

        let data = include_str!("../target/view.html");
        html::parse_view(&data)
    }

    pub async fn download(&self, url: Url) -> Result<(Vec<u8>, String)> {
        self.rate_limit(&url).await;

        let timer = Instant::now();
        let result = self.download_inner(url.clone()).await;
        let elapsed = timer.elapsed();
        self.event(
            chrono::Utc::now(),
            "download",
            Event::Download {
                url: url.to_string(),
                error: result.as_ref().err().map(|e| e.to_string()),
                elapsed: elapsed.as_secs_f64(),
            },
        );
        match result {
            Ok(data) => Ok(data),
            Err(err) => {
                tracing::error!("failed to download data from {}: {:?}", url, err);
                Err(err)
            }
        }
    }

    async fn download_inner(&self, url: Url) -> Result<(Vec<u8>, String)> {
        let fetch = |url: Url| async move {
            let response = self
                .inner
                .get(url)
                .header("Accept", "*/*; q=0.9")
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(anyhow::anyhow!(
                    "failed to fetch URL: {}",
                    response.status()
                ));
            }

            let content_type = response
                .headers()
                .get("Content-Type")
                .and_then(|v| v.to_str().ok())
                .context("failed to get content type")?
                .to_string();

            let data = response.bytes().await?;
            Ok((data.to_vec(), content_type))
        };

        self.retry_fetch(url, fetch).await
    }
}
