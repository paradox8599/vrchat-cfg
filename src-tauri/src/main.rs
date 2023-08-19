// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


pub mod cfg;
pub mod ops;
pub mod err;
pub mod traits;

use err::AppError;
use cfg::Config;
use traits::Json;

#[tauri::command]
fn greet() -> Result<String, AppError> {
    let mut cfg = Config::load().unwrap();
    if let Some(size) = &mut cfg.cache_size {
        *size -= 1;
    }
    cfg.save().unwrap();
    Ok(cfg.to_json())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
