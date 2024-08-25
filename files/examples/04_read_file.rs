use std::io::Read;

/// 读取文件内容
/// `Read` 提供了`read`方法，表示具有可读取特征。
/// `File` 对象实现了`Read`特征，所以可以被读取。
fn main() {
    // 打开一个文件
    let mut file = std::fs::File::open("README.md").unwrap();
    // 定义一个缓冲区，大小为10字节
    let mut buffer = [0; 10];
    // 从文件中读取10个字节，放入缓冲区
    let n = file.read(&mut buffer).unwrap();
    // 将字节数组 转换为字符串
    let s = std::str::from_utf8(&buffer[..n]).unwrap();
    println!("read content: {}",s);
}