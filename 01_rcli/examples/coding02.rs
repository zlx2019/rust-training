use std::fs;
use anyhow::Result;
/// 通过 anyhow 来处理错误.
fn main() -> Result<()>{
    let content = read_file("assets/juventus.csv")?;
    println!("{}",content);

    Ok(())
}


/// `read_file` 以字符串形式，读取文件内容
fn read_file(file_path: &str) -> Result<String>{
    Ok(fs::read_to_string(file_path)?) // 这里会将 std::io:Error 转换为 anyhow Error
}
