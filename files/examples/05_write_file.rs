use std::io::Write;

/// 写入文件内容
/// `Write` 提供了`write`方法，表示具有可写入特征。
/// `File` 对象实现了`Read`特征，所以可以被写入。
fn main() {
    // 打开或创建一个文件
    let mut file = std::fs::File::create("note.txt").unwrap();
    let mut content = "Hello world";
    // 向文件中写入数据
    if let Ok(_) = file.write(content.as_bytes()){
        println!("写入成功.");
    }
}