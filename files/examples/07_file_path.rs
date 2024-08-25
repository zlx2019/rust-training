//! `std::path::Path` 主要用于表示目录路径，它封装了路径字符串，并提供了多种方法来提取路径的各个部分内容
//! 如文件名、扩展名、父目录等信息

fn main() {
    // 构建文件所在路径
    let path = std::path::Path::new("foo.txt");
    if path.exists() {
        // 文件存在，文件基本信息如下:
        // 文件名(文件名 + 扩展名) `foo.txt`
        println!("文件全名: {:?}", path.file_name().unwrap());
        // 文件名
        println!("文件名前缀: {}", path.file_stem().unwrap().to_str().unwrap());
        // 文件扩展名 `txt`
        println!("文件后缀: {}", path.extension().unwrap().to_str().unwrap());
        // 路径类型
        println!("是否为绝对路径: {}", path.is_absolute());
        println!("是否为相对路径: {}", path.is_relative());
    }else {
        println!("{:?} 文件不存在!", path.file_name().unwrap());
    }
    // let meta_data = std::fs::metadata("foo.txt").unwrap();
}