use std::fs;

use anyhow::Context;


/// anyhow::Error 可以说是一个万能Error，它支持绝大多数Error到本身的转换
fn main() -> Result<(), anyhow::Error>{
    // 想打开的文件
    let filename = "non-existent-file.txt";

    // 打开文件，如果打开失败则会返回一个Err(std::io::Error)
    // 而 `anyhow::Error` 默认支持该错误的转换，所以可以直接向上传播
    fs::File::open(filename).with_context(|| format!("Can not open file: {}", filename))?;
    // 通过 with context, 可以为产生错误时添加一些额外的附加信息，便于调试
    Ok(())
}

