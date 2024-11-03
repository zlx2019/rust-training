/// `std::path:Path` 并不包含文件的物理信息，可以结合`std::fs::Metadata`模块来获取文件的元数据信息
use chrono::{DateTime, Local};
// use std::os::unix::fs::PermissionsExt;

fn main() {
    // 构建文件路径
    // let path = std::path::Path::new("foo.txt");
    // if path.exists() {
    //     // 获取文件的原数据信息
    //     // let meta_data = fs::metadata(path).unwrap();
    //     let meta_data = path.metadata().unwrap();
    //     println!("文件名: {}", path.to_str().unwrap());
    //     println!("文件大小: {}", meta_data.len());

    //     // 文件类型
    //     println!("文件类型: {:?}", meta_data.file_type());
    //     println!("is dir: {}", meta_data.is_dir());
    //     println!("is symlink: {}", meta_data.is_symlink());
    //     println!("is file: {}", meta_data.is_file());

    //     // 文件创建 && 修改时间
    //     let created: DateTime<Local> = DateTime::from(DateTime::<Local>::from(meta_data.created().unwrap()));
    //     let updated: DateTime<Local> = DateTime::from(DateTime::<Local>::from(meta_data.modified().unwrap()));
    //     println!("文件创建时间: {}", created.format("%Y-%m-%d %H:%M:%S").to_string());
    //     println!("文件修改时间: {}", updated.format("%Y-%m-%d %H:%M:%S").to_string());

    //     // 获取文件权限信息
    //     let permission = meta_data.permissions();
    //     println!("权限是否为只读: {}", permission.readonly());
    //     // 解析为 Unix 系统文件权限
    //     let mode = permission.mode();
    //     println!(
    //         "权限 (rwx): {}{}{}",
    //         if mode & 0o400 != 0 { "r" } else { "-" },  // 用户读权限
    //         if mode & 0o200 != 0 { "w" } else { "-" },  // 用户写权限
    //         if mode & 0o100 != 0 { "x" } else { "-" },  // 用户执行权限
    //     );
    //     println!(
    //         "权限 (rwx): {}{}{}",
    //         if mode & 0o040 != 0 { "r" } else { "-" },  // 组读权限
    //         if mode & 0o020 != 0 { "w" } else { "-" },  // 组写权限
    //         if mode & 0o010 != 0 { "x" } else { "-" },  // 组执行权限
    //     );
    //     println!(
    //         "权限 (rwx): {}{}{}",
    //         if mode & 0o004 != 0 { "r" } else { "-" },  // 其他用户读权限
    //         if mode & 0o002 != 0 { "w" } else { "-" },  // 其他用户写权限
    //         if mode & 0o001 != 0 { "x" } else { "-" },  // 其他用户执行权限
    //     );
    // }
}