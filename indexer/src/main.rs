use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context;
use axum::Extension;
use axum::extract::Path;
use axum::extract::Query;
use axum::routing::get;
use clap::Parser;
use kura_indexer::Result;
use kura_indexer::health::HealthRequest;
use kura_indexer::health::HealthResponse;
use kura_indexer::releases;
use rate_limiter::RateLimiter;
use request_tracker::RequestTracker;
use reqwest::Url;
use tokio::sync::Mutex;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;

mod cache;
mod cli;
mod client;
mod mirror;
mod rate_limiter;
mod request_tracker;

#[axum::debug_handler]
async fn releases_recent_handler(
    Query(request): Query<releases::RecentRequest>,
) -> Result<releases::RecentResponse> {
    request.validate()?;

    let releases = vec![
        releases::Release::builder("1", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::Movie))
            .add_tag("tag1".to_string())
            .build(),
        releases::Release::builder("2", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::TV))
            .add_tag("tag2".to_string())
            .build(),
    ];

    Ok(releases::RecentResponse {
        since: chrono::Utc::now(),
        until: chrono::Utc::now(),
        releases,
    })
}

#[axum::debug_handler]
async fn releases_search_handler(
    Query(request): Query<releases::SearchRequest>,
) -> Result<releases::SearchResponse> {
    request.validate()?;

    let releases = vec![
        releases::Release::builder("1", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::Movie))
            .add_tag("tag1".to_string())
            .build(),
        releases::Release::builder("2", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::TV))
            .add_tag("tag2".to_string())
            .build(),
    ];

    Ok(releases::SearchResponse { releases })
}

#[axum::debug_handler]
async fn health_handler(Query(_): Query<HealthRequest>) -> Result<HealthResponse> {
    let uptime = chrono::Duration::seconds(3600);
    let response = HealthResponse {
        service: "kura-indexer".to_string(),
        version: "1.0.0".to_string(),
        commit: "abc123".to_string(),
        status: "ok".to_string(),
        uptime,
    };
    Ok(response)
}

pub type ClientExt = Arc<Mutex<client::Client>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::Cli::parse();
    let config = cli::load_config(&cli.config)?;
    tracing::info!("loaded config: {:?}", config);

    let url = Url::parse(&config.nyaa.url).context("Failed to parse Nyaa URL")?;
    tracing::info!("nyaa url: {:?}", url);

    let request_tracker = RequestTracker::new(
        config.api.request_tracker_db.clone(),
    );
    let client = client::Client::builder(url)
        .timeout(
            config
                .nyaa
                .timeout
                .unwrap_or(std::time::Duration::from_secs(30)),
        )
        .cache_dir(config.api.cache_dir)
        .cache_size((config.api.cache_size_mb * 1024.0 * 1024.0) as u64)
        .rate_limiter(RateLimiter::new(
            config.nyaa.window_requests.unwrap_or(10),
            config
                .nyaa
                .window_size
                .unwrap_or(std::time::Duration::from_secs(60)),
        ))
        .request_tracker(request_tracker.clone())
        .build();
    let client = Arc::new(Mutex::new(client));

    let app = axum::Router::new()
        .route("/api/v1/releases/recent", get(releases_recent_handler))
        .route("/api/v1/releases/search", get(releases_search_handler))
        .route("/api/v1/health", get(health_handler));
    let app = mirror::routes(app, config.mirror.map_or(false, |m| m.enabled));

    let app = app
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(Extension(client))
        .layer(Extension(request_tracker));

    let listener = tokio::net::TcpListener::bind(config.api.listen_addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
