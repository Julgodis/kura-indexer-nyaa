use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use indexer::{NyaaContext, NyaaIndexer};
use kura_indexer::server::ServerBuilder;
use periodic::NyaaPeriodic;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NyaaConfig {
    url: String,
    #[serde(with = "humantime_serde")]
    update_interval: Duration,
    requests_per_second: usize,
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

    let context = NyaaContext {
        update_interval: config.nyaa.update_interval,
        db_path,
    };

    let url = Url::parse(&config.nyaa.url)
        .unwrap_or_else(|_| panic!("Failed to parse URL: {}", config.nyaa.url));

    let fetcher = NyaaPeriodic::new(context.clone(), config.nyaa.update_interval, url);

    fetcher.start();

    let builder = ServerBuilder::new(config.kura.listen_addr)
        .with_indexer::<NyaaIndexer>(context)
        .with_default_routes();

    let builder = if let Some(frontend) = config.frontend {
        if cfg!(feature = "frontend") {
            frontend::routes(builder)
        } else {
            tracing::warn!(
                "Frontend feature is not enabled. Please enable the frontend feature in Cargo.toml."
            );
            builder
        }
    } else {
        builder
    };

    builder.start().await.expect("Failed to start server");
}
