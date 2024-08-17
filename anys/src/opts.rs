use std::{fmt::Display, path::Path, str::FromStr};
use clap::Parser;
use anyhow::{Ok, Result};


/**
 * 终端命令行参数体
 */
#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct CommandLine{
    /// 子命令
    #[command(subcommand)]
    pub sub_command: Option<SubCommand>,
}

/**
 * 子命令枚举，每个子命令都代表一种处理分支.
 */
#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Csv 文件处理工具
    #[command(name = "csv", about = "Show CSV, or CSV to other formats.")]
    Csv(CsvOpts),
    /// 随机密码生成工具
    #[command(name = "rand", about = "Generate a random password")]
    GenPassword(GenPasswordOpts),
}


/// 密码生成命令参数
#[derive(Debug, Parser)]
pub struct GenPasswordOpts{
    /// 生成的密码长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// 允许出现大写字符
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    /// 允许出现小写字符
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    /// 允许出现数字
    #[arg(long, default_value_t = true)]
    pub number: bool,

    /// 允许出现符号
    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

/// CSV 相关命令行参数.
/// short：表示允许使用 简写前缀，如 input 属性，使用 -i 指定;
/// long：表示允许使用完整名称指定 -input
/// default_value：指定默认值.
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

/// 文件校验函数
fn file_verify(filename: &str) -> Result<String>{
    if !Path::new(filename).exists() {
        // 文件不存在.
        return Err(anyhow::anyhow!("File does not exists."));
    }
    Ok(filename.into())
}