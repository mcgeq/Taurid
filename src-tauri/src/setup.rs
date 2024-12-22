// -----------------------------------------------------------------------------
//              Copyright (C) 2024 Mcge. All rights reserved.
//
//  Author:         mcge
//  Email:          <mcgeq@outlook.com>
//  File:           setup.rs
//  Description:    Application Starting
//  Create   Date:  2024-12-21 15:35:07
//  Last Modified:  2024-12-22 14:55:59
//  Modified   By:  mcge <mcgeq@outlook.com>
// -----------------------------------------------------------------------------

use std::path::PathBuf;

use time::macros::{format_description, offset};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::{
    mglobal::{
        load_app_global_config, project_root, write_app_global_config, APP_GLOBAL_CONFIG,
        PROJECT_ROOT_DIR,
    },
    utils::mcge_files::McgeUtils,
};

fn setup_logging() -> WorkerGuard {
    // root_dir
    let project_path = project_root();

    let time_fmt =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");
    // 本地时间 东八区
    let timer = OffsetTime::new(offset!(+8), time_fmt);
    // 格式
    let formatter = fmt::format()
        .with_timer(timer)
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_source_location(true);

    // 过滤
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("trace,hyper=off,reqwest=off,thirtyfour=off,tao=off,tokio=off,scraper=off,html5ever=off,selectors=off,os_info=off")
    });

    // console
    let console_layer = fmt::layer()
        .event_format(formatter.clone())
        .pretty()
        .with_writer(std::io::stdout);
    // file
    let mut logs_dir = PathBuf::new();
    logs_dir.push(project_path);
    logs_dir.push("logs");
    let info_file_appender = rolling::daily(logs_dir, "info.log");
    let (no_blocking_appender, _guard) = tracing_appender::non_blocking(info_file_appender);
    let info_file_layer = fmt::layer()
        .with_ansi(false)
        .event_format(formatter.clone())
        .with_writer(no_blocking_appender);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(console_layer)
        .with(info_file_layer)
        .init();

    _guard
}

fn setup_project_root() {
    // 初始化根目录
    PROJECT_ROOT_DIR.get_or_init(|| McgeUtils::project_root_path().unwrap_or_default());
}

fn setup_app_global_config() {
    write_app_global_config();

    APP_GLOBAL_CONFIG.get_or_init(|| load_app_global_config());
}

pub fn setup() -> WorkerGuard {
    setup_project_root();

    setup_app_global_config();

    setup_logging()
}
