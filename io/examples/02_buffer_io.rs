use std::io::{BufRead, Write};

/// Rust默认提供的`Read`和`Write` trait，默认是基于字节的，不带有缓冲区，所以会对系统调用持续的频繁调用，导致效率
///极其低下，为了解决这个问题，`std::io`模块附带了两个缓冲区：`BufReader`和`BufWriter`，它们包装了读取器和写入器
///这两个包装器使用了缓冲区，可以减少系统调用次数。


/// 使用 BufReader 读取文件
fn using_buf_reader() -> std::io::Result<()>{
    let file = std::fs::File::open("foo.txt")?;
    // 将 文件对象 包装为 缓冲读取器
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = String::new();
    // 按行读取缓冲区内的所有数据，直到EOF返回 Ok(0)
    while reader.read_line(&mut buffer)? != 0 {
        println!("{buffer}");
    }
    Ok(())
}


/// 使用 BufWriter 向文件写入数据
fn using_buf_writer() -> std::io::Result<()> {
    let  file = std::fs::File::create("foo.txt")?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write("Hello World!".as_bytes())?;
    writer.flush()?; // 将缓冲数据写入到文件
    Ok(())
}

fn main() {
    using_buf_writer().unwrap();
    using_buf_reader().unwrap();
}