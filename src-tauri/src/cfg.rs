use crate::{err::AppError, traits::Json};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub cache_directory: Option<PathBuf>,
    pub cache_size: Option<u16>,
    pub cache_expiry_delay: Option<u16>,
    #[serde(rename = "disableRichPresence")]
    pub disable_rich_presence: Option<bool>,
    pub camera_res_height: Option<u16>,
    pub camera_res_width: Option<u16>,
    pub screenshot_res_height: Option<u16>,
    pub screenshot_res_width: Option<u16>,
    pub picture_output_folder: Option<PathBuf>,
    pub picture_output_split_by_date: Option<bool>,
    pub fpv_steadycam_fov: Option<u8>,
}

impl Json for Config {
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    fn from_json(json: &str) -> Result<Self, AppError> {
        match serde_json::from_str(json) {
            Ok(c) => Ok(c),
            Err(_) => Err(AppError::ConfigInvalidFormat),
        }
    }
}
