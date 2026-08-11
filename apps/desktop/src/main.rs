#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::Manager;

#[cfg(target_os = "windows")]
fn check_webview2() -> Result<(), String> {
    use std::path::Path;

    // Check common WebView2 installation locations
    let candidates = [
        // System-wide (HKLM) - 64-bit
        r"C:\Program Files (x86)\Microsoft\EdgeWebView\Application",
        // System-wide (HKLM) - 32-bit
        r"C:\Program Files\Microsoft\EdgeWebView\Application",
        // Per-user (HKCU)
        r"C:\Users",
    ];

    // Check registry for WebView2 runtime
    let check_registry = || -> bool {
        use std::process::Command;
        let output = Command::new("reg")
            .args([
                "query",
                r"HKLM\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
                "/v",
                "pv",
            ])
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                return true;
            }
        }
        let output = Command::new("reg")
            .args([
                "query",
                r"HKCU\SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}",
                "/v",
                "pv",
            ])
            .output();
        if let Ok(out) = output {
            if out.status.success() {
                return true;
            }
        }
        false
    };

    // Check if any WebView2 version directory exists
    for candidate in &candidates {
        if candidate.ends_with("Users") {
            // Skip the broad user directory check - registry is more reliable
            continue;
        }
        if Path::new(candidate).exists() {
            if let Ok(entries) = std::fs::read_dir(candidate) {
                for entry in entries.flatten() {
                    if entry.file_name().to_string_lossy().starts_with("1") {
                        return Ok(());
                    }
                }
            }
        }
    }

    if check_registry() {
        return Ok(());
    }

    Err("Microsoft Edge WebView2 Runtime is not installed. Nexivora requires WebView2 to run. Please install it from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/".to_string())
}

#[cfg(not(target_os = "windows"))]
fn check_webview2() -> Result<(), String> {
    Ok(())
}

fn main() {
    // Check WebView2 availability on Windows before starting the app
    #[cfg(target_os = "windows")]
    {
        if let Err(msg) = check_webview2() {
            // Show a message box instead of silently crashing
            let _ = rfd::MessageDialog::new()
                .set_title("Nexivora")
                .set_description(&msg)
                .set_level(rfd::MessageLevel::Error)
                .show();
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Log startup for debugging
            #[cfg(debug_assertions)]
            eprintln!("Nexivora setup: starting");

            if let Some(window) = app.get_webview_window("main") {
                if let Err(e) = window.show() {
                    eprintln!("Warning: failed to show main window: {}", e);
                }
                #[cfg(debug_assertions)]
                eprintln!("Nexivora setup: main window shown");
            } else {
                eprintln!("Warning: main window not found during setup");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Nexivora!", name)
}