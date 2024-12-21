// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <ge942812@gmail.com>
// File:           mcge_toml.rs
// Description:    Read and Write toml files.
// Create   Date:  2024-10-08 15:02:16
// Last Modified:  2024-12-21 19:13:55
// Modified   By:  mcge <mcgeq@outlook.com>
// ----------------------------------------------------------------------------

use serde::Deserialize;
use std::fs;
use std::path::Path;
use tracing::info;

pub struct McgeConfigToml;

impl McgeConfigToml {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<ConfigGlobal, Box<dyn std::error::Error>> {
        info!("读取配置文件路径: [{:?}]", path.as_ref());
        let content = fs::read_to_string(path)?;
        let config: ConfigGlobal = toml::de::from_str(&content)?;
        info!("配置文件读取成功: [{:?}]", config);
        Ok(config)
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ConfigGlobal {
    pub window: McgeWindow,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct McgeWindow {
    pub main_width: u32,
    pub main_height: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct McgEmail {
    pub stmp_server: String,
    pub from: String,
    pub to: String,
    pub username: String,
    pub password: String,
}
