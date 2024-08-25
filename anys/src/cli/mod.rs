//! cli 模块
//!

use std::path::Path;
use clap::Parser;

mod csv;
mod password;
mod http;
mod base64;

// 向外部导出的资源
pub use self::{csv::*, password::*, base64::*};

/**
 * 程序命令行启动参数体
 */
#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct CommandLine{
    #[command(subcommand)]
    pub sub_command: Option<SubCommand>,
}

/**
 * 子命令枚举，每个子命令都代表一种处理分支.
 */
#[derive(Debug, Parser)]
pub enum SubCommand {
    /// csv 文件处理
    #[command(name = "csv", about = "Show CSV, or CSV to other formats.")]
    Csv(CsvOpts),
    /// 随机密码生成
    #[command(name = "rand", about = "Generate a random password")]
    GenPassword(GenPasswordOpts),
    /// Base64 编解码处理
    #[command(subcommand, name = "base64", about = "base64 codec processing")]
    Base64(Base64SubCommand),
}


/// 文件是否存在校验
pub fn verify_input_file(filename: &str) -> Result<String, &'static str>{
    if Path::new(filename).exists() || filename == "-" {
        Ok(filename.into())
    }else {
        // 文件不存在.
        // return Err(anyhow::anyhow!("File does not exists"));
        Err("File does not exists")
    }
}


/// 支持的文件扩展类型
const JSON_EXT: &str = "json";
const CSV_EXT: &str = "csv";
const YAML_EXT: &str = "yaml";
const TOML_EXT: &str = "toml";

/// 校验文件类型是否为支持的.
pub fn verify_file_type(file_path: & str) -> Result<String, &'static str>{
    let path = Path::new(file_path);
    if !path.exists() {
        // 文件不存在
        return  Err("File does not exists");
    }
    // 过滤扩展名是否支持
    path.extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| matches!(*ext, CSV_EXT | JSON_EXT | TOML_EXT | YAML_EXT))
        .map(|_| file_path.into())
        .ok_or("Unsupported or unrecognized file type")
}


/// 单元测试模块
#[cfg(test)]
mod tests{
    use super::*;
    /// `file_verify` 函数单元测试
    #[test]
    fn test_file_verify(){
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("*"), Err("File does not exists"));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}