//! Csv 命令 相关处理

use std::fs::{self};
use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use crate::cli::OutputFormat;



/// 解析`input`文件内容，转换为`format`格式，并写入到`ouput`文件中.
///  - `input`:  输入文件
///  - `output`: 输出文件
///  - `format`: 输出的数据格式
/// 
/// #Examples
/// 
/// ```
/// use anys::{process_csv, OutputFormat};
/// _ = process_csv("./a.csv", "a.json", OutputFormat::Json);
/// 
/// ```
pub fn process_csv(input: &str, output: &str, format: OutputFormat)-> Result<()>{
    // 读取输入文件，解析为对应的实体列表
    let mut reader = Reader::from_path(input)?;
    // 获取 csv headers
    let headers =  reader.headers()?.clone();
    // 将每一行内容，解析为 JSON Value 格式
    let values = reader.records()
        .map(|item|item.unwrap())
        .map(|record| headers.iter()
            // 将 header 与 record的迭代器合并
            .zip(record.iter()) 
            .collect::<Value>())
        .collect::<Vec<Value>>();

    // 根据不同的输出类型，进行解析和转换
    let content = match format{
        OutputFormat::Json => serde_json::to_string_pretty(&values)?,
        OutputFormat::Yaml => serde_yaml::to_string(&values)?,
    };
    fs::write(output, content)?;
    Ok(())
}




//  #[allow(dead_code)]
// 读取输入文件，解析为 Player 结构，写入到输出文件.
// pub fn process_csv_player(input: &str, output: &str)-> Result<()>{
//     // 读取输入文件，解析为对应的实体列表
//     let players = Reader::from_path(input)?
//         .deserialize()
//         .map(|item|item.unwrap())
//         .collect::<Vec<Player>>();
//     // 转换为JSON格式，写入到输出文件
//     let json = serde_json::to_string_pretty(&players)?;
//     fs::write(output, json)?;
//     Ok(())
// }



// 读取 csv 格式的文件，解析后反序列化为指定的结构体.
// fn read_csv_file (file_path: &str) -> Result<()>{
//     // 打开文件，获取句柄
//     let file = File::open(file_path)?;
//     // 获取文件输出流
//     let mut reader = csv::Reader::from_reader(file);
//     // 迭代文件输出流，输出每一行csv结构内容
//     for record in reader.deserialize(){
//         let player: Player = record?;
//         println!("{:?}",player);
//         // println!("{}",player.to_json()?);
//     }
//     Ok(())
// }
