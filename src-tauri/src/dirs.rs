use std::path::Path;

use directories::UserDirs;
use fs_extra::{
    dir::{CopyOptions, TransitProcessResult},
    TransitProcess,
};

use crate::err::AppError;

pub fn get_cfg_dir() -> Result<String, AppError> {
    let user_dir = UserDirs::new().ok_or(AppError::PathNotFound("User Dir".to_string()))?;

    let vrchat_path = user_dir
        .home_dir()
        .join("AppData")
        .join("LocalLow")
        .join("VRChat")
        .join("vrchat");

    match vrchat_path {
        p if p.is_dir() => Ok(p
            .to_str()
            .ok_or(AppError::PathNonUTF8(p.to_string_lossy().to_string()))?
            .to_string()),
        p => Err(AppError::PathNotFound(p.to_string_lossy().to_string())),
    }
}

pub fn move_with_progress<P, Q, F>(
    from: P,
    to: Q,
    options: &CopyOptions,
    on_progress: F,
) -> Result<u64, AppError>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess) -> TransitProcessResult,
{
    let from = from.as_ref();
    fs_extra::move_items_with_progress(&[from], to, options, on_progress).map_err(|e| {
        AppError::MoveFilesFailed(format!(
            "{}: {}",
            e,
            from.file_name().unwrap_or_default().to_string_lossy(),
        ))
    })
}
