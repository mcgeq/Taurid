// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <mcgeq@outlook.com>
// File:           mgerror.rs
// Description:    Custom error
// Create   Date:  2024-11-08 15:47:17
// Last Modified:  2024-12-21 16:31:04
// Modified   By:  mcge <mcgeq@outlook.com>
// -----------------------------------------------------------------------------

use std::io;

use chrono::ParseError;
use thiserror::Error;
use tokio::task::JoinError;

pub type McgResult<T> = Result<T, McgError>;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum McgError {
    // 文件未找到
    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("data not found: {0}")]
    DataNotFound(String),

    #[error("invalid input")]
    InvalidInput,

    #[error(transparent)]
    IoError(#[from] io::Error),

    // 其他未知错误
    #[error("unknown error: {0}")]
    UnknowError(String),

    #[error("Failed driver start: {0}")]
    DriverStartFailed(String),

    // 数据库
    #[error("Database init failed: {0}")]
    DatabaseInitFailed(String),

    #[error("Database connect failed: {0}")]
    DatabaseConnectFailed(String),

    #[error("Database operation failed: {0}")]
    OperationFailed(String),

    #[error(transparent)]
    TokioJoinError(#[from] JoinError),

    #[error(transparent)]
    DateParseFailed(#[from] ParseError),

    // 配置相关的错误
    #[error("Failed to load config")]
    ConfigLoadFailed,

    // email
    #[error(transparent)]
    EmailFailed(#[from] lettre::transport::smtp::Error),
}
