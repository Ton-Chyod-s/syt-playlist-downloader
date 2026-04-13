mod commands;
mod process;
mod spotify;
mod types;
mod validation;

use std::sync::{Arc, Mutex};
use types::DownloadState;
use commands::{cancel_download, download_playlist};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DownloadState {
            pid: Arc::new(Mutex::new(None)),
            cancelled: Arc::new(Mutex::new(false)),
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            download_playlist,
            cancel_download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
