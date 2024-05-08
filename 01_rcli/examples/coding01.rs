use std::fs;


/// 通过 std::fd 标准库，读取文件内容.
fn main() {
    let content: String = fs::read_to_string("assets/juventus.csv")
        .expect("Error reading file");
    println!("{}",content);
}



/// `read_file` 以字符串形式，读取文件内容
fn _read_file<P> (file_path: P) -> String
where P: AsRef<std::path::Path>
{
    std::fs::read_to_string(file_path).expect("Failed to read file")
}
