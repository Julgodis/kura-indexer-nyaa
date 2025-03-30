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

    async fn process(context: &NyaaContext, url: Url) -> anyhow::Result<()> {
        let (data, cached) = context.client.fetch_list(url).await?;
        tracing::trace!("fetched data: {:#?}", data);
        tracing::debug!("found {} items", data.len());

        if !cached {
            context.add_items(&data)?;
        }
        Ok(())
    }

    async fn fetch(context: NyaaContext, url: Url) {
        tracing::info!("fetching data periodically...");

        match Self::process(&context, url.clone()).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("error fetching data: {:?}", e);
            }
        }
    }
}
