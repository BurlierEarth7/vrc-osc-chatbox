use std::{
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Deserializer, Serialize};

use crate::util::{error::AppError, mode::Mode};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(deserialize_with = "default_on_error")]
    pub display_mode: Mode,
    #[serde(deserialize_with = "default_on_error")]
    pub players: String,
    #[serde(default = "default_bind")]
    pub bind_address: SocketAddr,
    #[serde(default = "default_host")]
    pub host_address: SocketAddr,
    #[serde(deserialize_with = "default_on_error")]
    pub sync_message: String,
    #[serde(deserialize_with = "default_on_error")]
    pub sync_refresh_interval_seconds: u64,
    #[serde(deserialize_with = "default_on_error")]
    pub swap_message: String,
    #[serde(deserialize_with = "default_on_error")]
    pub send_immediately: bool,
    #[serde(deserialize_with = "default_on_error")]
    pub notify_on_send: bool,
    #[serde(deserialize_with = "default_on_error")]
    pub date_format: String,
}

fn default_on_error<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(T::deserialize(deserializer).unwrap_or_default())
}

// Custom defaults for sockets, since they do not have a default

fn default_bind() -> SocketAddr {
    "0.0.0.0:0".parse().unwrap()
}

fn default_host() -> SocketAddr {
    "127.0.0.1:9000".parse().unwrap()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_mode: Mode::Sync,
            players: "spotify,vlc,mpv,mpd".into(),
            bind_address: "0.0.0.0:0".parse().unwrap(),
            host_address: "127.0.0.1:9000".parse().unwrap(),
            sync_message: "{{title}} - {{artist}} [{{position}}/{{length}}]".into(),
            sync_refresh_interval_seconds: 5,
            swap_message: "Now Playing: {{title}} - {{artist}}".into(),
            send_immediately: true,
            notify_on_send: false,
            date_format: "+%c".into()
        }
    }
}

impl Config {
    /// Load user TOML config
    pub fn load() -> Result<Self, AppError> {
        let path = Self::config_path();

        if !path.exists() {
            let default = Self::default();
            default.write(&path)?;
            return Ok(default);
        }

        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn load_or_default() -> Self {
        match Self::load() {
            Ok(config) => config,

            Err(e) => {
                eprintln!("Failed to load config, using default as fallback until reloaded\n{e}");

                Self::default()
            }
        }
    }

    // Write user TOML config
    fn write(&self, path: &Path) -> Result<(), AppError> {
        // Create configuration directory
        if let Some(parent) = path.parent() {
            eprintln!("No configuration detected!");
            println!(
                "A default configuration has been generated at: {}",
                Self::config_path().to_string_lossy()
            );
            fs::create_dir_all(parent)?;
        }

        // Write to config
        let toml = toml::to_string_pretty(self)?;
        fs::write(path, toml)?;
        Ok(())
    }

    // Get expected config path
    pub fn config_path() -> PathBuf {
        let project_dirs = ProjectDirs::from("ca", "burlierearth7", "vrc-osc-chatbox")
            .expect("Could not determine configuration directory");

        project_dirs.config_dir().join("config.toml")
    }
}
