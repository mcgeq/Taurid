// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <mcgeq@outlook.com>
// File:           lib.rs
// Description:    Application start main
// Create   Date:  2024-12-22 15:23:02
// Last Modified:  2024-12-23 19:42:13
// Modified   By:  mcge <mcgeq@outlook.com>
// ----------------------------------------------------------------------------
mod common;
mod mgcommand;
mod mgerror;
mod mglobal;
#[cfg(mobile)]
mod mobiles;
mod setup;
mod utils;

use setup::setup;
use tauri::{AppHandle, Manager};
use tracing::{debug, info};

use crate::mgcommand::{greet, logmg};

#[cfg(mobile)]
use mobiles::window;

#[tauri::command]
async fn show_main_window(app: AppHandle) {
    debug!("Showing main window");

    let main_window = app.get_webview_window("main").unwrap();
    main_window.show().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _guard = setup();
    info!("All configurations have benn loaded.");
    info!("Application starting...");
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init());
    
    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
                if let Some(w) = app.get_webview_window("main") {
                    info!("Application instance already running, focusing existing window.");
                    w.set_focus().unwrap();
                }
            }))
            .plugin(tauri_plugin_updater::Builder::new().build())
            .plugin(tauri_plugin_window_state::Builder::new().build())
            .plugin(tauri_plugin_process::init());
    }
    
    #[cfg(mobile)]
    {
        #[cfg_attr(mobile, tauri::mobile_entry_point)]
        builder = builder.setup(|app| {
            window::setup_mobile_window(app.handle())?;
            Ok(())
        });
    }
    
    builder
        .invoke_handler(tauri::generate_handler![show_main_window, logmg, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
