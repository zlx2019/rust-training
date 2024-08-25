//! base64 命令处理

use std::fs::File;
use std::io::Read;
use base64::{engine::general_purpose::{URL_SAFE_NO_PAD, STANDARD}, Engine as _};
use crate::cli::Base64Format;

/// 将输入进行 base64 编码，并且返回
pub fn base64_encode(input: &str, format: Base64Format) -> anyhow::Result<String>{
    // 读取输入内容
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    }else {
        // 输入的是文件，读取文件内容
        Box::new(File::open(input)?)
    };
    let mut buffer = Vec::new();
    let _ = reader.read_to_end(&mut buffer)?;
    // 将内容进行编码
    let output = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buffer),
    };
    // println!("base64 encode {} -> {}", String::from_utf8(buffer)?, &output);
    Ok(output)
}

/// 将输入内容进行 base64 解码，并且返回
pub fn base64_decode(input: &str, format: Base64Format) -> anyhow::Result<String>{
    // 读取输入内容
    let mut reader: Box<dyn Read> =  match input {
        "-" => Box::new(std::io::stdin()),
        _ => Box::new(File::open(input)?)
    };
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    // 解码
    let output_bytes = match format {
        Base64Format::Standard => STANDARD.decode(&buffer.trim())?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buffer.trim())?,
    };
    let output =  String::from_utf8(output_bytes)?;
    // println!("base64 decode {} -> {}", &input, &output);
    Ok(output)
}