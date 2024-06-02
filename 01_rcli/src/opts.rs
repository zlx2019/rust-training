use std::{fmt::Display, path::Path, str::FromStr};
use clap::Parser;
use anyhow::{Ok, Result};

/// 终端命令参数解析模块.
/// 将终端输入的启动参数，解析为 Options::SubCommand.


#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Options{
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// 命令行参数枚举，可以包含多种命令参数
#[derive(Debug, Parser)]
pub enum SubCommand {
    // CSV 命令参数
    #[command(name = "csv", about = "Csv Info.")]
    Csv(CsvOpts),
}

/// CSV 相关命令行参数.
/// short：表示允许使用 简写前缀，如 input 属性，使用 -i 指定;
/// long：表示允许使用完整名称指定 -input
/// default_value：指定默认值.
#[derive(Debug, Parser)]
pub struct CsvOpts{
    // 输入的文件路径
    #[arg(short, long, value_parser = file_verify)]
    pub input: String,

    // 输出的文件路径
    #[arg(short, long)]
    pub output: Option<String>,

    // 输出的文件格式
    #[arg(short, long, value_parser = parser_format, default_value = "json")]
    pub format: OutputFormat,

    // CSV 分隔符，默认为 ','
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // 是否显示 CSV 文件头信息.
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

/// 文件校验函数
fn file_verify(filename: &str) -> Result<String>{
    if !Path::new(filename).exists() {
        // 文件不存在.
        return Err(anyhow::anyhow!("File does not exists."));
    }
    Ok(filename.into())
}