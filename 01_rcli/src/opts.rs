use std::path::Path;
use clap::Parser;
use anyhow::Result;

/// 终端命令参数解析模块.
/// 将终端输入的启动参数，解析为 Options::SubCommand.


#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Options{
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// 命令行参数枚举，可以包含多种命令参数
#[derive(Debug, Parser)]
pub enum SubCommand {
    // CSV 命令参数
    #[command(name = "csv", about = "Csv Info.")]
    Csv(CsvOpts),
}

/// CSV 相关命令行参数.
/// short：表示允许使用 简写前缀，如 input 属性，使用 -i 指定;
/// long：表示允许使用完整名称指定 -input
/// default_value：指定默认值.
#[derive(Debug, Parser)]
pub struct CsvOpts{
    // 输入的 CSV 文件
    #[arg(short, long, value_parser = file_verify)]
    pub input: String,

    // 输出的文件
    #[arg(short, long, default_value = "assets/output.json")]
    pub output: String,

    // CSV 分隔符，默认为 ','
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    // 是否显示 CSV 文件头信息.
    #[arg(long, default_value_t = true)]
    pub header: bool,
}


/// 文件校验函数
fn file_verify(filename: &str) -> Result<String>{
    if !Path::new(filename).exists() {
        // 文件不存在.
        return Err(anyhow::anyhow!("File does not exists."));
    }
    Ok(filename.into())
}
