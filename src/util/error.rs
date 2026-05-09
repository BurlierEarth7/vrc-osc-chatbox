use std::process::ExitStatus;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Command `{cmd}` failed with exit status {status}: {stderr}")]
    CommandFail {
        cmd: String,
        status: ExitStatus,
        stderr: String,
    },

    #[error("No active media players")]
    NoActivePlayers,

    #[error("OSC encoding error: {0}")]
    OscEncode(#[from] rosc::OscError),

    #[error("UTF-8 error")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    #[error("Float Parse Error")]
    ParseFloat(#[from] std::num::ParseFloatError),
}
