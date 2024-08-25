//! 标准IO
//!  - `std::io::Stdin`  标准输入
//!  - `std::io::Stdout` 标准输出
//!  - `std::io::Stderr` 标准错误输出
//!
//! 以上三种对象均实现了 `Read`、`Write` 特征，为标准IO
//!
//! 下面演示如何使用标准IO


use std::io::Write;

fn main() -> std::io::Result<()>{
    // 从标准输入读取一行数据
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;

    // 输出到标准输出
    std::io::stdout().write(line.as_bytes())?;

    // 输出到标准错误输出
    std::io::stderr().write_all(line.as_bytes())?;
    Ok(())
}