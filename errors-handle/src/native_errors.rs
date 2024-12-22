#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Display;

/// 自定义原生Error, 不依赖于 anyhow 或 thiserror 第三方库
///  - 通过手动实现 From 特征完成其他错误到 `NativeBusinessError` 的转换。
#[derive(Debug)]
pub enum NativeBusinessError{
    // IO 相关错误，是对标准IO库的错误包装
    IOError(std::io::Error),
    // serdeJson 相关错误
    JSONSerializationError(serde_json::error::Error),
    // 整数解析错误
    IntegerParseError(std::num::ParseIntError),
    // 系统未知错误
    SysError(String)
}


/// 实现 `std::io::Error` 转换为  `NativeBusinessError`
/// 只有这样才能将 `std::io::Error` 自动向上传播为 `NativeBusinessError`
impl From<std::io::Error> for NativeBusinessError{
    fn from(err: std::io::Error) -> Self {
        NativeBusinessError::IOError(err)
    }
}


impl Display for NativeBusinessError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main(){

}

