use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use clap::Parser;

use super::verify_input_file;

/**
 * base64 命令, 对指定的内容进行 base64 编解码.
 */
#[derive(Debug, Parser)]
pub enum Base64SubCommand{
    /// base64 编码
    #[command(name = "encode", about = "base64 encoder")]
    Encode(Base64EncodeOpts),
    /// base64 解码
    #[command(name = "decode", about = "base64 decoder")]
    Decode(Base64DecodeOpts),
}

/**
 * base64 编码指令参数
 */
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// 要进行base64编码的内容
    #[arg(value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    /// 编码格式，指定解析函数
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format
}

/**
 * base64 解码指令参数
 */
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// 要进行base64解码的内容
    #[arg(value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    /// 解码格式
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format

}

/**
 * Base64 编码/解码 配置选项
 */
#[derive(Debug, Clone, Copy)]
pub enum Base64Format{
    // 标准字符集
    Standard,
    // 可以用在URL等场景的编码字符集
    UrlSafe
}

/// 为 clap 实现 &str 到 `Base64Format` 的转换
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error>{
    format.parse()
}

/// 为 `Base64Format` 实现 FromStr 特征
/// `&str` -> `Base64Format`
impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "url_safe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format"))
        }
    }
}

/// 实现 `Base64Format` -> `&str` 的转换
impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "url_safe",
        }
    }
}


/// `Base64Format` 输出打印时，转换为`&str`类型
impl fmt::Display for Base64Format{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}