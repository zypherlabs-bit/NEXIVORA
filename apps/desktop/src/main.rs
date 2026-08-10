//! Nexivora Desktop Application Entry Point

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use nexivora_desktop::AppState;

fn main() {
    env_logger::init();
    log::info!("Starting Nexivora Office Suite Desktop Application...");

    tauri::Builder::default()
        .manage(AppState::new())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
