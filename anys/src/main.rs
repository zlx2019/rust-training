
use ansi_term::Color::Green;
use anyhow::{Ok, Result};
use clap::Parser;
use anys::{gen_password, process_csv, CommandLine, SubCommand, Base64SubCommand, base64_encode, base64_decode};


/// 程序入口
fn main() -> Result<()>{
    // 解析终端命令行参数
    let command_line = CommandLine::parse();
    if command_line.sub_command.is_none() {
        return Ok(());
    }
    // 处理子命令
    match command_line.sub_command.unwrap() {
        // Csv command handle
        SubCommand::Csv(opt) => {
            let output = if let Some(output) = opt.output{
                output
            }else{
                let in_path = &opt.input[..opt.input.rfind('.').unwrap()];
                format!("{}.{}",in_path, opt.format)
            };
            println!("{}", output);
            process_csv(&opt.input, &output, opt.format)?
        },
        // rand command handle
        SubCommand::GenPassword(opt) => {
            let password = gen_password(opt.length, opt.uppercase,
                opt.lowercase, opt.number, opt.no_symbol);
            eprintln!("generated password: {}", Green.paint(password.unwrap()));
        }
        // base64 command handle
        SubCommand::Base64(sub_cmd) => {
            let output = match sub_cmd {
                Base64SubCommand::Encode(opts) => {
                    base64_encode(&opts.input, opts.format)?
                }
                Base64SubCommand::Decode(opts) => {
                    base64_decode(&opts.input, opts.format)?
                }
            };
            println!("{output}");
        }
    }
    Ok(())
}
