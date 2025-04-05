use std::{fs, net::SocketAddr, path::{Path, PathBuf}};

use clap::Parser;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub static_dir: PathBuf,
    pub cors_allow_everyone: Option<bool>,
    pub request_tracker_db: PathBuf,
    pub mirror: Vec<MirrorConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MirrorConfig {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: MirrorType,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    pub url: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_addr: Option<String>,
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

    pub cache_dir: PathBuf,
    pub cache_size_mb: f64,
    #[serde(with = "humantime_serde")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_duration: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MirrorType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "adult")]
    Adult,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "nyaa-mirror.toml")]
    pub config: PathBuf,
}

pub fn load_config(config_path: impl AsRef<Path>) -> anyhow::Result<Config> {
    let config_path = config_path.as_ref();
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
