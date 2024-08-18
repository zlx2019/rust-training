#![allow(dead_code)]

use std::path::PathBuf;
use clap::Parser;
use crate::cli::file_verify;




/**
 * HTTP 子命令
 */
#[derive(Debug, Parser)]
pub enum HTTPSubCommand {
    #[command(about = "Serve a directory over HTPP")]
    Serve(HTTPServeOpts),
}

/**
 * 
 * HTTP 服务参数
 */
#[derive(Debug, Parser)]
pub struct HTTPServeOpts{
    /// 文件目录
    #[arg(short, long, value_parser = file_verify, default_value = ".")]
    pub path: PathBuf,
    /// 服务端口号
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}