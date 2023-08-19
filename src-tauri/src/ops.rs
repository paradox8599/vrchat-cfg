// use std::path::Path;

// use fs_extra::{
//     dir::{CopyOptions, TransitProcessResult},
//     TransitProcess,
// };

// use crate::err::AppError;

// pub fn move_with_progress<P, Q, F>(
//     from: P,
//     to: Q,
//     options: &CopyOptions,
//     on_progress: F,
// ) -> Result<u64, AppError>
// where
//     P: AsRef<Path>,
//     Q: AsRef<Path>,
//     F: FnMut(TransitProcess) -> TransitProcessResult,
// {
//     let from = from.as_ref();
//     fs_extra::move_items_with_progress(&[from], to, options, on_progress).map_err(|e| {
//         AppError::MoveFilesFailed(format!(
//             "{}: {}",
//             e,
//             from.file_name().unwrap_or_default().to_string_lossy(),
//         ))
//     })
// }
