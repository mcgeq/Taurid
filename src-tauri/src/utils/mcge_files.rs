// -----------------------------------------------------------------------------
//    Copyright (C) 2024 mcge. All rights reserved.
// Author:         mcge
// Email:          <mcgeq@outlook.com>
// File:           mcge_files.rs
// Description:    Custom files
// Create   Date:  2024-09-22 09:38:39
// Last Modified:  2024-12-21 16:21:46
// Modified   By:  mcge <mcgeq@outlook.com>
// ----------------------------------------------------------------------------

use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use tracing::error;

use crate::mgerror::{McgError, McgResult};

#[derive(Debug)]
pub struct McgeUtils;

#[allow(dead_code)]
impl McgeUtils {
    // 系统路径分隔符
    pub fn separator() -> &'static str {
        if cfg!(windows) {
            "\\"
        } else {
            "/"
        }
    }
    // 获取项目根目录
    pub fn project_root_path() -> io::Result<String> {
        let mut root_dir = PathBuf::new();
        if let Ok(exe_path) = env::current_exe() {
            root_dir.push(exe_path);
        } else {
            root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        };

        if McgeUtils::exists(&root_dir) {
            if let Some(p) = root_dir.parent() {
                Ok(p.to_string_lossy().into_owned())
            } else {
                error!("未找到根目录.");
                Err(io::Error::new(io::ErrorKind::NotFound, "未找到根目录"))
            }
        } else {
            error!("未找到根目录.");
            Err(io::Error::new(io::ErrorKind::NotFound, "未找到根目录"))
        }
    }

    // 创建目录
    pub fn create_dir(dir_path: &Path) -> bool {
        if !McgeUtils::exists(dir_path) {
            if fs::create_dir_all(dir_path).is_ok() {
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    // 拼接
    pub fn join_dir(paths: &[&str]) -> PathBuf {
        let mut path = PathBuf::new();
        for p in paths {
            path.push(p);
        }
        path
    }

    /// 获取指定文件的父级目录，根据传入的层级数字获取对应的父目录
    pub fn get_parent_directory(file_path: &str, level: usize) -> McgResult<String> {
        let abs_path = Path::new(file_path).canonicalize()?;
        let mut current_path = abs_path.clone();

        for _ in 0..=level {
            if let Some(parent) = current_path.parent() {
                current_path = parent.to_path_buf();
            } else {
                return Err(McgError::from(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "层级数超过目录范围",
                )));
            }
        }

        Ok(current_path.to_string_lossy().to_string())
    }

    // 读取文件
    pub fn read_file_to_string<P: AsRef<Path>>(file_path: P) -> McgResult<String> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    // 获取文件名和扩展
    pub fn get_file_name_extension(file_path: &str) -> McgResult<String> {
        let path = Path::new(file_path);
        // 文件名
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => return Err(McgError::FileNotFound("获取文件名失败".to_string())),
        };

        // 扩展名
        let extension = match path.extension() {
            Some(ext) => ext.to_string_lossy().to_string(),
            None => return Err(McgError::FileNotFound("获取扩展名失败".to_string())),
        };
        let mut f = String::from(file_name);
        f.push('.');
        f.push_str(&extension);
        Ok(f)
    }

    pub fn write_to_file(file_path: &str, content: &str) -> McgResult<()> {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
    // 判断目录是否存在
    pub fn exists(dir_path: &Path) -> bool {
        dir_path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_dir() {
        let p = McgeUtils::join_dir(vec!["d", "dev"].as_ref());
        assert_eq!(Some("d\\dev"), p.as_path().to_str());
    }

    #[test]
    fn test_creatte_dir() {
        let _d = McgeUtils::create_dir(Path::new("D:/ddd/sss"));
        assert!(_d);
    }

    #[test]
    fn test_project_root_dir() {
        let rp = McgeUtils::project_root_path();
        match rp {
            Ok(p) => assert_eq!(
                String::from("D:\\projects\\2024\\p-rust\\mcge-daily-details"),
                p
            ),
            Err(e) => assert_eq!(io::ErrorKind::NotFound, e.kind()),
        }
    }
}
