use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;

/// Rust 打开文件的几种方式
/// `File::open` 以可读模式打开指定文件，文件不存在则会返回Error.
/// `OpenOptions` 以更详细的方式打开文件，可选标志如下:
///     read: bool,
///     write: bool,
///     append: bool,
///     truncate: bool, - 截断之前的数据
///     create: bool,    - 不存在则创建，已存在则打开文件
///     create_new: bool, - 不存在则创建，已存在则返回Error

fn main() {
    // `File::open` 方法默认以可读模式打开文件
    let file_res = File::open("xxx.md");
    // 如果打开的文件不存在，则会返回Error
    if let Err(e) = file_res {
        // 打开文件错误，错误信息处理
        // 原生错误码
        let code = e.raw_os_error().unwrap();
        // 错误类型
        let kind = e.kind();
        // 错误消息
        let msg = e.to_string();
        if kind == io::ErrorKind::NotFound {
            println!("抱歉，文件不存在");
        }else {
            println!("打开文件错误: Code: {}  \
                              Kind: {:?}  \
                              Msg: {}",
                     code, kind, msg);
        }
    }

    // `OpenOptions` 可用于配置打开文件时的选项与标志
    // File::open 和 File::create 等方法都是它的包装
    // 如下，打开一个文件，如果不存在则创建，并且可读可写，以追加模式
    let mut new_file = OpenOptions::new()
                            .create(true)
                            .read(true)
                            .append(true)
                            .open("new.txt").unwrap();
    // 写入数据
    new_file.write_all(b"Hello World\n").unwrap();
}