use std::{fs, net::SocketAddr, path::Path};

use serde::Deserialize;

use crate::util::{error::AppError, mode::Mode};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub display_mode: Mode,
    pub players: String,
    pub bind_address: SocketAddr,
    pub host_address: SocketAddr,
    pub sync_message: String,
    pub sync_refresh_interval_seconds: u64,
    pub on_change_message: String,
    pub send_immediately: bool,
    pub notify_on_send: bool,
    pub meta_format: String
}

impl Config {
    /// Load a TOML config file from the given path
    /// 
    /// * `path` - The specified path to the TOML config
    pub fn load(path: &str) -> Result<Self, AppError> {
        let content = fs::read_to_string(Path::new(path))?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}