use crate::{err::AppError, traits::Json};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use directories::UserDirs;

// https://docs.vrchat.com/docs/configuration-file

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_directory: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_size: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_expiry_delay: Option<u16>,

    #[serde(rename = "disableRichPresence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rich_presence: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_res_height: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_res_width: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_res_height: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_res_width: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_output_folder: Option<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_output_split_by_date: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
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

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let cfg_file_path = Self::get_file_path()?;
        let path_str = cfg_file_path.to_string_lossy().to_string();
        let cfg_file_str = std::fs::read_to_string(cfg_file_path)
            .map_err(|_| AppError::ConfigReadFailed(path_str))?;
        Self::from_json(&cfg_file_str)
    }

    pub fn save(&self) -> Result<(), AppError> {
        let cfg_file_path = Self::get_file_path()?;
        let cfg_file_str = self.to_json();
        std::fs::write(&cfg_file_path, cfg_file_str)
            .map_err(|_| AppError::ConfigReadFailed(cfg_file_path.to_string_lossy().to_string()))
    }

    fn get_dir_path() -> Result<PathBuf, AppError> {
        let user_dir = UserDirs::new().ok_or(AppError::PathNotFound("User Dir".to_string()))?;

        let vrchat_path = user_dir
            .home_dir()
            .join("AppData")
            .join("LocalLow")
            .join("VRChat")
            .join("vrchat");

        let path_str = vrchat_path.to_string_lossy().to_string();
        match vrchat_path {
            p if p.is_dir() && p.is_absolute() => Ok(p),
            _ => Err(AppError::PathNotDir(path_str)),
        }
    }

    pub fn get_file_path() -> Result<PathBuf, AppError> {
        let cfg_dir = Self::get_dir_path()?;
        let cfg_file_path = cfg_dir.join("config.json");
        match cfg_file_path {
            p if p.is_file() => Ok(p),
            _ => Err(AppError::PathNotFile(
                cfg_file_path.to_string_lossy().to_string(),
            )),
        }
    }

    // setters

    pub fn set_cache_directory(&mut self, path: PathBuf) -> Result<(), AppError> {
        if path.is_dir() && path.is_absolute() {
            self.cache_directory = Some(path);
            return Ok(());
        }
        Err(AppError::CachePathInvalid(
            path.to_string_lossy().to_string(),
        ))
    }

    pub fn set_cache_size(&mut self, size: u16) -> Result<(), AppError> {
        if size < 20 {
            return Err(AppError::CacheSizeTooSmall(size));
        }
        self.cache_size = Some(size);
        Ok(())
    }

    pub fn set_cache_expiry_delay(&mut self, delay: u16) -> Result<(), AppError> {
        if delay < 30 {
            return Err(AppError::CacheExpiryDelayTooSmall(delay));
        }
        self.cache_expiry_delay = Some(delay);
        Ok(())
    }

    // More setters to be added
}
