use std::{fmt::Display, str::FromStr};
use clap::Parser;
use anyhow::{Ok, Result};

use super::file_verify;



/**
 * csv 子命令参数
 */
#[derive(Debug, Parser)]
pub struct CsvOpts{
    /// 输入的文件
    #[arg(short, long, value_parser = file_verify)]
    pub input: String,

    /// 输出的文件
    #[arg(short, long)]
    pub output: Option<String>,

    /// 输出的文件格式
    #[arg(short, long, value_parser = parser_format, default_value = "json")]
    pub format: OutputFormat,

    /// CSV 分隔符，默认为 ','
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// 是否显示 CSV 文件头信息.
    #[arg(long, default_value_t = true)]
    pub header: bool,
}


/// 输出的文件格式.
#[derive(Debug,Clone, Copy)]
pub enum OutputFormat{
    Json,
    Yaml,
}

/// format 参数校验函数
fn parser_format(format: &str) -> Result<OutputFormat>{
    // 解析 format 参数，并且转换为 OutputFormat 枚举
    format.parse::<OutputFormat>()
}

/// 通过 From 特征，实现 OutputFormat -> &str 的转换
impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

/// 通过 FromStr 特征，实现 &str -> OutputFormat 的尝试转换，无法转换则返回Err
impl FromStr for OutputFormat{
    type Err = anyhow::Error;
    fn from_str(value: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            v => anyhow::bail!("Invalid format: {}.",v)
        }
    }    
}

impl Display for OutputFormat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", Into::<&str>::into(*self))
    }
}
