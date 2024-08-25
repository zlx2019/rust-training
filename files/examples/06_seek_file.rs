use std::io::{Read, Seek, SeekFrom};

///  移动文件内容指针
/// `Seek` 特征提供了一个可以在字节流中移动的光标，流通常具有固定大小，允许相对于任一端或当前偏移进行查找。
/// `File` 对象自然也实现了该特征，所以可以移动操作文件的位置。

fn main() {
    let mut file = std::fs::File::open("README.md").unwrap();
    // 通过 Seek 读取文件最后10个字节
    let mut buffer = [0; 10];
    // 移动到文件末尾 - 10 的位置.
    file.seek(SeekFrom::End(-10)).unwrap();
    if let Ok(n) = file.read(&mut buffer){
        println!("last 10 bytes: {:?}", String::from_utf8_lossy(&buffer[..n]));
    }
}