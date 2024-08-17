// 项目 Library 管理

// 定义模块
mod opts;
mod player;

// 命令处理相关模块，包含其他子模块
mod process;

// 导出模块
pub use opts::{CommandLine, SubCommand, CsvOpts};
pub use player::Player;
pub use process::process_csv;
