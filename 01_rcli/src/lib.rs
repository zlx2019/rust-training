// 项目 Library 管理

// 定义模块
// opts - 终端命令解析模块
mod opts;

// 数据处理模块
mod process;

// player 实体模块
mod player;

// 导出模块
pub use opts::{Options, SubCommand, CsvOpts};
pub use player::Player;
pub use process::process_csv;
