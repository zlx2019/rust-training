use clap::Parser;

use super::file_verify;

/**
 * base64 命令, 对指定的内容进行 base64 编解码.
 */
#[derive(Debug, Parser)]
pub enum Base64SubCommand{    
    #[command(name = "e", about = "base64 encoder")]
    Encode(Base64EncodeOpts),
    #[command(name = "d", about = "base64 decoder")]
    Decode(Base64DecodeOpts),
}

/**
 * 编码命令参数
 */
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(value_parser = file_verify, default_value = "-")]
    pub input: String
}

/**
 * 解码命令参数
 */
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(value_parser = file_verify, default_value = "-")]
    pub input: String
}