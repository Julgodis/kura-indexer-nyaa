use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use reqwest::Url;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{cache::Cache, rate_limiter::RateLimiter, request_tracker::RequestTracker};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQuery {
    #[serde(default)]
    #[serde(rename = "p")]
    pub page: Option<usize>,
    #[serde(default)]
    #[serde(rename = "c")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(rename = "f")]
    pub filter: Option<String>,
    #[serde(default)]
    #[serde(rename = "s")]
    pub sort: Option<String>,
    #[serde(default)]
    #[serde(rename = "o")]
    pub order: Option<String>,
    #[serde(default)]
    #[serde(rename = "q")]
    pub query: Option<String>,
}

impl ListQuery {
    pub fn remove_defaults(mut self) -> Self {
        if self.page == Some(1) {
            self.page = None;
        }
        if self.category == Some("0_0".to_string()) {
            self.category = None;
        }
        if self.filter == Some("0".to_string()) {
            self.filter = None;
        }
        if self.sort == Some("id".to_string()) {
            self.sort = None;
        }
        if self.order == Some("desc".to_string()) {
            self.order = None;
        }
        if self.query == Some("".to_string()) {
            self.query = None;
        }
        self
    }
}

#[derive(Debug)]
pub struct Client {
    mirror_id: String,
    url: Url,
    interface: Option<String>,
    local_addr: Option<String>,
    user_agent: String,
    timeout: Duration,
    cache: Arc<Mutex<Cache>>,
    rate_limiter: RateLimiter,
    request_tracker: Option<RequestTracker>,
    cache_duration: Duration,
}

impl Client {
    pub fn builder(mirror_id: impl AsRef<str>, url: Url) -> ClientBuilder {
        ClientBuilder::new(mirror_id, url)
    }

    pub async fn list(&self, query: &ListQuery) -> anyhow::Result<Vec<nyaa_parser::ListItem>> {
        tracing::debug!("fetching list from {:?}", self.url.to_string());

        let begin = std::time::Instant::now();

        let url = self.url.clone();
        if let Some(value) = self.cache.lock().await.get(&url, &query) {
            self.request_tracker
                .as_ref()
                .map(|tracker| tracker.track_request_cached(&self.mirror_id, &url, &query));
            return Ok(value);
        }

        self.rate_limiter.acquire().await;

        let url = self.url.clone();
        let mut client = reqwest::Client::builder()
            .connection_verbose(true)
            .user_agent(self.user_agent.clone())
            .timeout(self.timeout);

        if let Some(local_addr) = &self.local_addr {
            client = client.local_address(Some(
                local_addr
                    .parse()
                    .context("failed to parse local address")?,
            ));
        }
        if let Some(interface) = &self.interface {
            client = client.interface(interface);
        }

        let client = client.build().context("failed to build HTTP client")?;

        let response = client
            .get(url.clone())
            .query(&query)
            .send()
            .await
            .context("failed to send request")?;
        let status = response.status();
        if status.is_success() {
            let content_type = response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_lowercase()
                .to_string();

            let body = response
                .text()
                .await
                .context("failed to read response body")?;
            let result = if content_type.contains("xml") {
                nyaa_parser::list::rss::parse(&body)?
            } else if content_type.contains("html") {
                let scheme = url.scheme();
                let host = url.host_str().unwrap_or("");
                let url_str = if let Some(port) = url.port() {
                    format!("{}://{}:{}", scheme, host, port)
                } else {
                    format!("{}://{}", scheme, host)
                };
                nyaa_parser::list::html::parse(&url_str, &body)?
            } else {
                return Err(anyhow::anyhow!(
                    "unsupported content type: {}",
                    content_type
                ));
            };

            let elapsed_time = begin.elapsed().as_secs_f64();
            self.request_tracker.as_ref().map(|tracker| {
                tracker.track_request(&self.mirror_id, &url, &query, true, elapsed_time)
            });

            self.cache
                .lock()
                .await
                .put(&url, query, self.cache_duration, &result);
            return Ok(result);
        } else {
            let error_body = response
                .text()
                .await
                .unwrap_or("failed to read error response body".to_string());

            let elapsed_time = begin.elapsed().as_secs_f64();
            self.request_tracker.as_ref().map(|tracker| {
                tracker.track_request(&self.mirror_id, &url, &query, false, elapsed_time)
            });

            return Err(anyhow::anyhow!(
                "request failed with status code {}:\n{}",
                status,
                error_body
            ));
        }
    }

    pub async fn view(&self, id: &str) -> anyhow::Result<nyaa_parser::View> {
        tracing::debug!("fetching view from {:?}", self.url.to_string());

        let begin = std::time::Instant::now();
        let url = self.url.clone();
        if let Some(value) = self.cache.lock().await.get(&url, &id) {
            self.request_tracker
                .as_ref()
                .map(|tracker| tracker.track_request_cached(&self.mirror_id, &url, &id));
            return Ok(value);
        }

        self.rate_limiter.acquire().await;

        let url = self.url.join(&format!("/view/{}", id))?;
        let mut client = reqwest::Client::builder()
            .connection_verbose(true)
            .user_agent(self.user_agent.clone())
            .timeout(self.timeout);

        if let Some(local_addr) = &self.local_addr {
            client = client.local_address(Some(
                local_addr
                    .parse()
                    .context("failed to parse local address")?,
            ));
        }
        if let Some(interface) = &self.interface {
            client = client.interface(interface);
        }

        let client = client.build().context("failed to build HTTP client")?;

        let response = client
            .get(url.clone())
            .send()
            .await
            .context("failed to send request")?;
        let status = response.status();
        if status.is_success() {
            let body = response
                .text()
                .await
                .context("failed to read response body")?;
            let scheme = url.scheme();
            let host = url.host_str().unwrap_or("");
            let url_str = if let Some(port) = url.port() {
                format!("{}://{}:{}", scheme, host, port)
            } else {
                format!("{}://{}", scheme, host)
            };
            let result = nyaa_parser::view::html::parse(&url_str, &body)
                .context("failed to parse response body")?;

            let elapsed_time = begin.elapsed().as_secs_f64();
            self.request_tracker.as_ref().map(|tracker| {
                tracker.track_request(&self.mirror_id, &url, &id, true, elapsed_time)
            });

            self.cache
                .lock()
                .await
                .put(&url, &("view", &id), self.cache_duration, &result);
            return Ok(result);
        } else {
            let error_body = response
                .text()
                .await
                .unwrap_or("failed to read error response body".to_string());

            let elapsed_time = begin.elapsed().as_secs_f64();
            self.request_tracker.as_ref().map(|tracker| {
                tracker.track_request(&self.mirror_id, &url, &id, false, elapsed_time)
            });

            return Err(anyhow::anyhow!(
                "request failed with status code {}:\n{}",
                status,
                error_body
            ));
        }
    }

    pub async fn magnet_link(&self, id: &str) -> anyhow::Result<String> {
        let view = self.view(id).await?;
        if let Some(magnet_link) = view.magnet_link {
            return Ok(magnet_link);
        }

        return Err(anyhow::anyhow!("failed to get magnet link for id {}", id));
    }
}

pub struct ClientBuilder {
    mirror_id: String,
    url: Url,
    interface: Option<String>,
    local_addr: Option<String>,
    user_agent: String,
    timeout: Duration,
    cache_dir: PathBuf,
    cache_size: u64,
    cache_duration: Duration,
    rate_limiter: RateLimiter,
    request_tracker: Option<RequestTracker>,
}

impl ClientBuilder {
    pub fn new(mirror_id: impl AsRef<str>, url: Url) -> Self {
        Self {
            mirror_id: mirror_id.as_ref().to_string(),
            url,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".into(),
            timeout: Duration::from_secs(30),
            cache_dir: PathBuf::from("cache"),
            cache_size: 64 * 1024 * 1024,
            cache_duration: Duration::from_secs(60 * 60),
            rate_limiter: RateLimiter::new(10, Duration::from_secs(1)),
            request_tracker: None,
            interface: None,
            local_addr: None,
        }
    }

    pub fn cache_dir(mut self, path: impl AsRef<Path>) -> Self {
        self.cache_dir = path.as_ref().to_path_buf();
        self
    }

    pub fn cache_size(mut self, size: u64) -> Self {
        self.cache_size = size;
        self
    }

    pub fn cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn rate_limiter(mut self, rate_limiter: RateLimiter) -> Self {
        self.rate_limiter = rate_limiter;
        self
    }

    pub fn local_addr(mut self, local_addr: impl Into<Option<String>>) -> Self {
        self.local_addr = local_addr.into();
        self
    }

    pub fn interface(mut self, interface: impl Into<Option<String>>) -> Self {
        self.interface = interface.into();
        self
    }

    pub fn request_tracker(mut self, request_tracker: RequestTracker) -> Self {
        self.request_tracker = Some(request_tracker);
        self
    }

    pub fn build(self) -> Client {
        let cache = Cache::new(self.cache_dir, self.cache_size).expect("failed to create cache");
        let cache = Arc::new(Mutex::new(cache));
        Client {
            mirror_id: self.mirror_id,
            url: self.url,
            interface: self.interface,
            local_addr: self.local_addr,
            user_agent: self.user_agent,
            timeout: self.timeout,
            cache,
            rate_limiter: self.rate_limiter,
            request_tracker: self.request_tracker,
            cache_duration: self.cache_duration,
        }
    }
}
