use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use indexer::{NyaaContext, NyaaIndexer, NyaaMode};
use kura_indexer::server::ServerBuilder;
use periodic::NyaaPeriodic;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub mod client;
pub mod data;
pub mod html;
pub mod indexer;
pub mod periodic;
pub mod rss;
pub mod url;

#[cfg(feature = "frontend")]
pub mod frontend;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "nyaa-indexer.toml")]
    config: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Config {
    kura: KuraConfig,
    nyaa: NyaaConfig,
    frontend: Option<FrontendConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct KuraConfig {
    listen_addr: SocketAddr,
    db_path: String,
    event_db_path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NyaaConfig {
    url: String,
    mode: NyaaMode,
    #[serde(with = "humantime_serde")]
    update_interval: Duration,
    requests_per_window: usize,
    #[serde(with = "humantime_serde")]
    window_size: Duration,
    #[serde(with = "humantime_serde")]
    connect_timeout: Duration,
    #[serde(with = "humantime_serde")]
    timeout: Duration,
    max_retries: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct FrontendConfig {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!(
        "{} {} starting...",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    // Parse command line arguments
    let cli = Cli::parse();

    // Load and parse configuration
    let config_content = fs::read_to_string(&cli.config)
        .unwrap_or_else(|_| panic!("Failed to read config file: {:?}", cli.config));

    let config: Config = toml::from_str(&config_content)
        .unwrap_or_else(|e| panic!("Failed to parse config file: {}", e));

    tracing::info!("Configuration:\n{:#?}", cli.config);

    let db_path = PathBuf::from(&config.kura.db_path);
    if !db_path.exists() {
        fs::create_dir_all(db_path.parent().unwrap())
            .unwrap_or_else(|_| panic!("Failed to create directory: {:?}", db_path));
    }

    rusqlite::Connection::open(&db_path)
        .unwrap_or_else(|_| panic!("Failed to open database: {}", config.kura.db_path))
        .execute_batch(include_str!("../schema.sql"))
        .unwrap_or_else(|_| panic!("Failed to execute schema.sql"));

    let event_db_path = PathBuf::from(&config.kura.event_db_path);
    if !event_db_path.exists() {
        fs::create_dir_all(event_db_path.parent().unwrap())
            .unwrap_or_else(|_| panic!("Failed to create directory: {:?}", event_db_path));
    }

    rusqlite::Connection::open(&event_db_path)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to open event database: {}",
                config.kura.event_db_path
            )
        })
        .execute_batch(include_str!("../event_schema.sql"))
        .unwrap_or_else(|_| panic!("Failed to execute event_schema.sql"));

    let min_interval = Duration::from_secs_f64(
        config.nyaa.window_size.as_secs_f64() / config.nyaa.requests_per_window as f64,
    );
    let min_interval = if min_interval < Duration::from_secs(1) {
        Duration::from_secs(1)
    } else {
        min_interval
    };
    let client = client::Client::new(
        min_interval,
        event_db_path.clone(),
        config.nyaa.connect_timeout,
        config.nyaa.timeout,
        config.nyaa.max_retries,
    )
    .expect("Failed to create client");

    let base_url = Url::parse(&config.nyaa.url)
        .unwrap_or_else(|_| panic!("Failed to parse URL: {}", config.nyaa.url));
    let context = NyaaContext {
        update_interval: config.nyaa.update_interval,
        db_path,
        event_db_path,
        client,
        base_url: base_url,
        mode: config.nyaa.mode,
    };

    let url = Url::parse(&config.nyaa.url)
        .unwrap_or_else(|_| panic!("Failed to parse URL: {}", config.nyaa.url));

    let fetcher = NyaaPeriodic::new(context.clone(), config.nyaa.update_interval, url);

    fetcher.start();

    let builder = ServerBuilder::new(config.kura.listen_addr)
        .with_indexer::<NyaaIndexer>(context)
        .with_default_routes();

    #[cfg(feature = "frontend")]
    let builder = if let Some(frontend) = config.frontend {
        frontend::routes(builder)
    } else {
        builder
    };

    #[cfg(not(feature = "frontend"))]
    if config.frontend.is_some() {
        tracing::warn!("Frontend feature is disabled. Ignoring frontend configuration.");
    }

    builder.start().await.expect("Failed to start server");
}
