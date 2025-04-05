use std::sync::Arc;

use axum::{Extension, Router};
use clap::Parser;
use cli::{MirrorConfig, MirrorType};
use rate_limiter::RateLimiter;
use request_tracker::RequestTracker;
use reqwest::Url;
use tokio::sync::Mutex;
use tower_http::{
    compression::CompressionLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

mod api;
mod cache;
mod cli;
mod client;
mod rate_limiter;
mod request_tracker;

#[derive(Debug, Clone)]
pub struct Mirror {
    pub config: MirrorConfig,
    pub api_url: Url,
    pub client: Arc<Mutex<client::Client>>,
}

impl Mirror {
    pub fn new(config: MirrorConfig, request_tracker: RequestTracker) -> anyhow::Result<Self> {
        let api_url = if let Ok(url) = Url::parse(&config.url) {
            url
        } else if let Ok(url) = Url::parse(&format!("http://{}", config.url)) {
            url
        } else {
            Url::parse(&config.url)?
        };

        let client = client::Client::builder(&config.id, api_url.clone())
            .timeout(config.timeout.unwrap_or(std::time::Duration::from_secs(30)))
            .cache_dir(config.cache_dir.clone())
            .cache_size((config.cache_size_mb * 1024.0 * 1024.0) as u64)
            .cache_duration(
                config
                    .cache_duration
                    .unwrap_or(std::time::Duration::from_secs(60)),
            )
            .rate_limiter(RateLimiter::new(
                config.window_requests.unwrap_or(10),
                config
                    .window_size
                    .unwrap_or(std::time::Duration::from_secs(60)),
            ))
            .request_tracker(request_tracker)
            .local_addr(config.local_addr.clone())
            .interface(config.interface.clone())
            .build();
        Ok(Self {
            config,
            api_url,
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub fn is_hidden(&self) -> bool {
        self.config.hidden.unwrap_or(false)
    }

    pub fn id(&self) -> &str {
        &self.config.id
    }

    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub fn ty(&self) -> MirrorType {
        self.config.ty.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MirrorExt {
    mirrors: Vec<Mirror>,
}

impl MirrorExt {
    pub fn iter(&self) -> impl Iterator<Item = &Mirror> {
        self.mirrors.iter()
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Mirror> {
        self.mirrors.iter().find(|mirror| mirror.id() == id)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = cli::Cli::parse();
    let config = cli::load_config(&cli.config)?;
    tracing::info!("loaded config: {:?}", config);

    let index_path = config.static_dir.join("index.html");

    let mut app = axum::Router::new()
        .route_service("/", ServeFile::new(index_path.clone()))
        .nest_service(
            "/api",
            Router::new()
                .route(
                    "/mirror/{mirror}/list",
                    axum::routing::get(api::mirror::list::handler),
                )
                .route(
                    "/mirror/{mirror}/view/{id}",
                    axum::routing::get(api::mirror::view::handler),
                )
                .route(
                    "/mirror/{mirror}/magnet/{id}",
                    axum::routing::get(api::mirror::magnet::handler),
                )
                .route("/mirror", axum::routing::get(api::mirror::handler))
                .route("/health", axum::routing::get(api::health::handler)),
        )
        .route_service("/{*path}", ServeFile::new(index_path))
        .nest_service("/static", ServeDir::new(config.static_dir))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    if config.cors_allow_everyone.unwrap_or(false) {
        app = app.layer(tower_http::cors::CorsLayer::very_permissive());
    }

    let request_tracker = request_tracker::RequestTracker::new(config.request_tracker_db.clone());
    let mext = MirrorExt {
        mirrors: config
            .mirror
            .iter()
            .map(|mirror_config| -> anyhow::Result<Mirror> {
                Mirror::new(mirror_config.clone(), request_tracker.clone())
            })
            .collect::<Result<Vec<_>, _>>()?,
    };

    let app = app.layer(Extension(mext)).layer(Extension(request_tracker));

    let listener = tokio::net::TcpListener::bind(config.listen_addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
