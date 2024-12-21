// -----------------------------------------------------------------------------
//              Copyright (C) 2024 Mcge. All rights reserved.
//
//  Author:         mcge
//  Email:          <mcgeq@outlook.com>
//  File:           mglog.rs
//  Description:    Custom log
//  Create   Date:  2024-12-21 23:15:03
//  Last Modified:  2024-12-21 23:25:04
//  Modified   By:  mcge  <mcgeq@outlook.com>
// -----------------------------------------------------------------------------

pub enum MgLogLevel {
    I,
    W,
    E,
    D,
    T,
}

impl MgLogLevel {
    pub fn parse_str(log_type: &str) -> Self {
        match log_type.to_uppercase().as_str() {
            "T" => MgLogLevel::T,
            "W" => MgLogLevel::W,
            "D" => MgLogLevel::D,
            "E" => MgLogLevel::E,
            _ => MgLogLevel::I,
        }
    }
}
