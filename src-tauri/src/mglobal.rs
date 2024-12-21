// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <mcgeq@outlook.com>
// File:           config.rs
// Description:    Load stock global config
// Create   Date:  2024-11-06 16:24:29
// Last Modified:  2024-12-21 17:31:38
// Modified   By:  mcge <mcgeq@outlook.com>
// -----------------------------------------------------------------------------

use std::sync::OnceLock;

use std::path::{Path, PathBuf};

use crate::utils::mcge_toml::McgeConfigToml;
use crate::utils::{mcge_files::McgeUtils, mcge_toml::ConfigGlobal};

use tracing::{error, info};

pub static PROJECT_ROOT_DIR: OnceLock<String> = OnceLock::new();
pub static APP_GLOBAL_CONFIG: OnceLock<Option<ConfigGlobal>> = OnceLock::new();

pub fn project_root_dir() -> String {
    McgeUtils::project_root_path().unwrap_or_default()
}

pub fn write_app_global_config() {
    let mut root_dir = PathBuf::from(project_root_dir());
    root_dir.push("config");
    if !McgeUtils::exists(&root_dir) {
        if McgeUtils::create_dir(&root_dir) {
            info!("配置文件目录创建成功.");
        } else {
            error!("配置文件目录创建失败.");
        }
    }
    root_dir.push("app-global");
    root_dir.set_extension("toml");

    let config = r#"
[window]
main_width = 1200
main_height = 800

[email]
stmp_server = 'smtp.163.com'
from = ''
to = ''
username = ''
password = ''
"#;
    if !McgeUtils::exists(&root_dir) {
        match McgeUtils::write_to_file(root_dir.to_path_buf().to_string_lossy().as_ref(), &config) {
            Ok(_) => info!("写配置文件成功"),
            Err(e) => error!("配置文件写失败: {:?}", e),
        }
    }
}

pub fn get_app_global_config(config_path: &str) -> Option<ConfigGlobal> {
    let root_path = Path::new(config_path);
    let mut config_file = root_path.to_path_buf();
    config_file.push("config");
    config_file.push("app-global");
    config_file.set_extension("toml");
    McgeConfigToml::load(config_file).ok()
}
