use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
#[serde(tag = "error", content = "value")]
pub enum AppError {
    #[error("Path contains non utf-8 chars: `{0}`")]
    PathNonUTF8(String),

    #[error("`{0}` path not found")]
    PathNotFound(String),

    #[error("Failed to move files: `{0}`")]
    MoveFilesFailed(String),

    #[error("Config file invalid format")]
    ConfigInvalidFormat,
}

impl AppError {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
