// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use err::AppError;

pub mod cfg;
pub mod dirs;
pub mod err;
pub mod traits;

#[tauri::command]
fn greet() -> Result<String, AppError> {
    dirs::get_cfg_dir()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
