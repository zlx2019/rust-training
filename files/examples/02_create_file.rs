use std::fs::File;
use std::io::Write;

/// 通过 `File::create` 创建一个文件
/// 该方法默认打开选项为: 不存在则创建，并且截断原有内容，以及开启写模式
/// ```
/// OpenOptions::new().write(true).create(true).truncate(true).open(path.as_ref())
/// ```
fn main(){
    // 在当前工作目录下创建 foo.txt 文件
    let file_res = File::create("./foo.txt");
    match file_res {
        Ok(mut file) => {
            // 创建成功后，获得打开的文件句柄
            println!("successful to create file");
            // 向文件写入内容
            file.write_all(b"Hello World").expect("write line failed");
        }
        Err(e) => {
            // 创建失败
            println!("failed to create file: {}", e);
        }
    }
}