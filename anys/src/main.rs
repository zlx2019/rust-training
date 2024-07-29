
use anyhow::{Ok, Result};
use clap::Parser;
use anys::{process_csv, Options, SubCommand};


/// Main fcuntion
fn main() -> Result<()>{
    // 解析终端参数.
    let opts = Options::parse();
    println!("{:?}",opts);

    // 处理终端参数.
    match opts.cmd {
        SubCommand::Csv(cmd) => {
            // 处理输出文件名称与类型
            let output = if let Some(output) = cmd.output{
                output
            }else{
                let in_path = &cmd.input[..cmd.input.rfind('.').unwrap()];
                format!("{}.{}",in_path, cmd.format)
            };

            process_csv(&cmd.input, &output, cmd.format)?
        },
    }
    Ok(())
}
