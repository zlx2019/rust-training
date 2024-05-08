use anyhow::{Ok, Result};
use clap::Parser;
use rcli::{process_csv, Options, SubCommand};


/// Main fcuntion
fn main() -> Result<()>{
    // 解析终端参数.
    let opts = Options::parse();
    println!("{:?}",opts);

    // 处理终端参数.
    match opts.cmd {
        SubCommand::Csv(csv) => process_csv(&csv.input, &csv.output)?,
    }
    Ok(())
}
