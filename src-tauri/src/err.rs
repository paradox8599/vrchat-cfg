use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
#[serde(tag = "error", content = "value")]
pub enum AppError {
    // Path errors
    #[error("Path contains non utf-8 chars: `{0}`")]
    PathNonUTF8(String),

    #[error("`{0}` path not found")]
    PathNotFound(String),

    #[error("`{0}` is not a file")]
    PathNotFile(String),

    #[error("`{0}` is not a directory")]
    PathNotDir(String),

    // Config file errors
    #[error("Config file invalid format")]
    ConfigInvalidFormat,

    #[error("Failed to read config file: `{0}`")]
    ConfigReadFailed(String),

    // Config restrictions
    #[error("Cache path is invalid: `{0}`")]
    CachePathInvalid(String),

    #[error("Cache size should be >= 20, but got: `{0}`")]
    CacheSizeTooSmall(u16),

    #[error("Cache expiry delay should be >= 30, but got: `{0}`")]
    CacheExpiryDelayTooSmall(u16),
}

impl AppError {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
