#![allow(unused_variables)]
#![allow(dead_code)]
/// ==========================
/// errors.rs 自定义错误 使用案例
/// ==========================


use std::fs;
use error_handle::BusinessError;
use serde_json::Value;


fn main() -> Result<(), BusinessError>{
    // 该函数返回的错误类型与主函数一致，所以可以直接通过?向上传播。
    fail_with_unknown_error()?;

    // 该函数返回的错误类型为 `std::io::Error`，与主函数的错误类型不一致
    // 但是通过 thiserror 将 `std::io::Error` 自动转换为了 `BusinessError::IOError`
    // 所以也可以直接向上传播     
    let file = fs::File::open("non-existent-file.txt")?;

    // `std::num::ParseIntError` 也可以自动转换为 `BusinessError::IntegerParseError` 
    let num = "123".parse::<usize>()?;

    // `serder_json::error::Error` 同样也实现了到 `BusinessError::JSONSerializationError` 的转换
    let user: Value = serde_json::from_str(r#"{"name": "zhangsan", "age": 18}"#)?;
    println!("{}", user);
    Ok(())
}



/// 该函数可能会产生未知的错误，返回 `BusinessError::SysError` 即可
fn fail_with_unknown_error() -> Result<(), BusinessError>{
    if false {
        // TODO
        unreachable!()
    }
    Err(BusinessError::SysError("unknown error".to_string()))
}