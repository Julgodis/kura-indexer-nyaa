use std::time::Duration;

use anyhow::Context;
use reqwest::{ClientBuilder, Url};

use crate::{data, html, indexer::NyaaContext, rss};

pub struct NyaaPeriodic {
    context: NyaaContext,
    duration: tokio::time::Duration,
    url: Url,
}

impl NyaaPeriodic {
    pub fn new(context: NyaaContext, duration: tokio::time::Duration, url: Url) -> Self {
        Self {
            context,
            duration,
            url,
        }
    }

    pub fn start(&self) {
        let context = self.context.clone();
        let duration = self.duration.clone();
        let url = self.url.clone();
        tokio::spawn(async move {
            loop {
                Self::fetch(context.clone(), url.clone()).await;
                tokio::time::sleep(duration).await;
            }
        });
    }

    async fn fetch_data(url: Url) -> anyhow::Result<Vec<data::Item>> {
        let client = ClientBuilder::new()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()?;

        let response = client
            .get(url)
            .header("Accept", "application/xml, text/html, */*; q=0.9")
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
        tracing::trace!("content type: {}", content_type);

        let data = response.text().await?;
        tracing::trace!("fetched data: {}", data);

        if content_type.contains("application/xml") {
            tracing::trace!("XML content type detected");
            rss::parse(&data)
        } else if content_type.contains("text/html") {
            tracing::trace!("HTML content type detected");
            html::parse_list(&data)
        } else {
            return Err(anyhow::anyhow!(
                "unsupported content type: {}",
                content_type
            ));
        }
    }

    async fn test_xml_data(_url: Url) -> anyhow::Result<Vec<data::Item>> {
        let data = include_str!("../target/test.xml");
        rss::parse(&data)
    }

    async fn test_html_data(_url: Url) -> anyhow::Result<Vec<data::Item>> {
        let data = include_str!("../target/list.html");
        html::parse_list(&data)
    }

    async fn process(context: &NyaaContext, url: Url) -> anyhow::Result<()> {
        let data = Self::test_html_data(url).await?;
        tracing::trace!("fetched data: {:#?}", data);
        tracing::debug!("found {} items", data.len());

        context.add_items(data)?;
        Ok(())
    }

    async fn fetch(context: NyaaContext, url: Url) {
        // Implement periodic fetching logic here
        tracing::info!("fetching data periodically...");

        match Self::process(&context, url.clone()).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("error fetching data: {:?}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
                tracing::warn!("trying another URL...");
            }
        }
    }
}
