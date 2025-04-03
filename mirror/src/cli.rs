use std::{fs, net::SocketAddr, path::{Path, PathBuf}};

use clap::Parser;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub listen_addr: SocketAddr,
    pub static_dir: PathBuf,
    pub cors_allow_everyone: Option<bool>,
    pub mirror: Vec<MirrorConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MirrorConfig {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: MirrorType,
    pub api_url: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(skip)]
    pub api_url_parsed: Option<reqwest::Url>,
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
