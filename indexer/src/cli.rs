use std::{fs, net::SocketAddr, path::{Path, PathBuf}};

use clap::Parser;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub api: ApiConfig,
    pub nyaa: NyaaConfig,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror: Option<MirrorConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub listen_addr: SocketAddr,
    pub cache_dir: PathBuf,
    pub cache_size_mb: f64,
    pub request_tracker_db: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NyaaConfig {
    pub url: String,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<std::time::Duration>,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<std::time::Duration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_requests: Option<usize>,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MirrorConfig {
    pub enabled: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "nyaa-indexer.toml")]
    pub config: PathBuf,
}

pub fn load_config(config_path: impl AsRef<Path>) -> anyhow::Result<Config> {
    let config_path = config_path.as_ref();
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
