use crate::types::*;

/// 获取一个目录下的所有文件名
pub fn list_files(path: &str) -> Result<Vec<String>>{
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|re| re.ok())
        .filter(|e| {
            e.file_type().map(|ft| ft.is_file()).unwrap_or(false)
        })
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if files.is_empty() {
        return Err("Cannot list empty folder".into());
    }
    Ok(files)
}