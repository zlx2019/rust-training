use std::path::Path;

///
/// cli 模块
/// 

use clap::Parser;


// 该模块下的子模块
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
    // base64
    #[command(subcommand)]
    Base64(Base64SubCommand),
}


/// 文件是否存在校验
pub fn file_verify(filename: &str) -> anyhow::Result<String>{
    if Path::new(filename).exists() || filename == "-" {
        Ok(filename.into())
    }else {
        // 文件不存在.
        return Err(anyhow::anyhow!("File does not exists."));
    }
}


