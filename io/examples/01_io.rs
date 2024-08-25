use std::io::Read;

/// `std::io` 模块包含了输入和输出功能。最核心的两个Traits: `Read`和`Write`，它们提供了最通用的输入和输出接口。
/// 大多数类型都实现了这两个特征，比如`fs::File`、`net::TcpStream`、甚至是一个`Vec<T>`。
///
/// 例如`Read`特征定义了一个`read`方法，表示读取(输出)，而`File`实现了`Read`，所以可以被读取
fn main() {
    // 打开一个文件
    let mut file = std::fs::File::open("README.md").unwrap();
    // 定义一个缓冲区，大小为10字节
    let mut buffer = [0; 10];
    // 从文件中读取10个字节，放入缓冲区
    let n = file.read(&mut buffer).unwrap();
    println!("read bytes: {:?}", &buffer[..n]);
}