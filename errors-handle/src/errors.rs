#![allow(unused_variables)]
#![allow(dead_code)]

/// 自定义业务错误类型
///  - 通过 thiserror 实现其他错误到 `BusinessError` 的转换
#[derive(Debug, thiserror::Error)]
pub enum BusinessError{
    // IO 相关错误，是对标准IO库的错误包装
    // 通过 thiserror 将 `std::io::Error` 转换成 `BusinessError::IOError`
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    // serdeJson 相关错误
    #[error("JSON Serialization error: {0}")]
    JSONSerializationError(#[from] serde_json::error::Error),

    // 整数解析错误
    #[error("parse error: {0}")]
    IntegerParseError(#[from] std::num::ParseIntError),

    // 系统未知错误
    #[error("system error: {0}")]
    SysError(String)
}



