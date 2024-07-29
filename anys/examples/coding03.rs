/// 基于 Clap 对命令行参数进行解析与映射.

use std::path::Path;
use clap::{Parser, arg};


#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Options{
    #[command(subcommand)]
    cmd: SubCommand,
}

/// 命令行参数体，可以包含多种命令参数
#[derive(Debug, Parser)]
enum SubCommand {
    // CSV 命令参数
    #[command(name = "csv", about = "Csv Info.")]
    Csv(CsvOpts),
}

/// CSV 相关命令行参数.
/// short：表示允许使用 简写前缀，如 input 属性，使用 -i 指定;
/// long：表示允许使用完整名称指定 -input
/// default_value：指定默认值.
#[derive(Debug, Parser)]
struct CsvOpts{
    // 输入的 CSV 文件
    #[arg(short, long, value_parser = file_verify)]
    input: String,

    // 输出的文件
    #[arg(short, long, default_value = "output.json")]
    output: String,

    // CSV 分隔符，默认为 ','
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    // 是否显示 CSV 文件头信息.
    #[arg(long, default_value_t = true)]
    header: bool,
}


/// 文件校验函数
fn file_verify(filename: &str) -> Result<String, &'static str>{
    if !Path::new(filename).exists() {
        // 文件不存在.
        return Err("File does not exists");
    }
    Ok(filename.into())
}

fn main(){
    // 命令行参数解析.
    let commands = Options::parse();
    println!("{:?}",commands);
}
