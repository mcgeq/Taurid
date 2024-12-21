// -----------------------------------------------------------------------------
//              Copyright (C) 2024 Mcge. All rights reserved.
//
//  Author:         mcge
//  Email:          <mcgeq@outlook.com>
//  File:           mgcommand.rs
//  Description:    Front command
//  Create   Date:  2024-12-21 18:32:31
//  Last Modified:  2024-12-21 23:35:55
//  Modified   By:  mcge  <mcgeq@outlook.com>
// -----------------------------------------------------------------------------

use tracing::{debug, error, info, trace, warn};

use crate::common::mglog::MgLogLevel;

// front call log
#[tauri::command(rename_all = "snake_case")]
pub fn logmg(lg_type: String, prefix: String, msg: String) {
    match MgLogLevel::parse_str(lg_type.as_str()) {
        MgLogLevel::T => trace!("[{}: {}]", prefix, msg),
        MgLogLevel::D => debug!("[{}: {}]", prefix, msg),
        MgLogLevel::E => error!("[{}: {}]", prefix, msg),
        MgLogLevel::W => warn!("[{}: {}]", prefix, msg),
        _ => info!("[{}: {}]", prefix, msg),
    }
}

#[tauri::command]
pub async fn greet(name: String) -> String {
    format!("Welcome {}", name)
}
