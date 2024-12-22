#![allow(unused_variables)]
#![allow(dead_code)]
/// ==========================
/// native_errors.rs 自定义基础错误 使用案例
/// ==========================

use std::fs;

use error_handle::NativeBusinessError;


 
/// 主函数返回错误类型为 `NativeBusinessError`
fn main() -> Result<(), NativeBusinessError>{
    println!("size of MyError is {}", std::mem::size_of::<NativeBusinessError>());

    // 该函数返回错误类型为 `std::io::Error`
    // 但是通过 From 实现了 `std::io::Error` 到 `NativeBusinessError::IOError`的转换
    // 所以可以直接向上传播。
    let file = fs::File::open("non-existent-file.txt")?;
    Ok(())
}
