// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <mcgeq@outlook.com>
// File:           window.rs
// Description:    About Mobile Windows Settings
// Create   Date:  2024-12-22 16:02:41
// Last Modified:  2024-12-22 16:08:59
// Modified   By:  mcge <mcgeq@outlook.com>
// ----------------------------------------------------------------------------

use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

pub fn setup_mobile_window(app: &AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(app, "main", WebviewUrl::default()).build()?;
    Ok(())
}
