//! Rust `std::fs` 该模块包含了操作本地文件系统的基本方法
//! `std::fs::File`结构体，提供了对文件系统上打开的文件的访问功能。
//! `File` 实现了 `Read`和`Write`特征，所以可以被读取或写入，具体取决于打开它时设置的选项。还实现了`Seek`来更改文件内部
//! 包含的逻辑游标。文件操作超出范围后会自动关闭。
//! 另外，`File` 默认不会对读写的数据进行缓冲，如果想提高效率，可以将文件包装在`BufReader`和`BufWriter`中。
use std::fs::*;

fn main() {

}