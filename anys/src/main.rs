
use anyhow::{Ok, Result};
use clap::Parser;
use anys::{process_csv, CommandLine, SubCommand};


/// 程序入口
fn main() -> Result<()>{
    // 解析终端命令行参数
    let command_line = CommandLine::parse();
    println!("{:?}", command_line);
    if command_line.sub_command.is_none() {
        return Ok(());
    }
    // 处理子命令
    match command_line.sub_command.unwrap() {
        SubCommand::Csv(opt) => {
            // Csv command handle
            let output = if let Some(output) = opt.output{
                output
            }else{
                let in_path = &opt.input[..opt.input.rfind('.').unwrap()];
                format!("{}.{}",in_path, opt.format)
            };
            process_csv(&opt.input, &output, opt.format)?
        },
        SubCommand::GenPassword(opt) => {
            // rand command handle
            println!("{:?}", opt);
        },
        
    }
    Ok(())
}
