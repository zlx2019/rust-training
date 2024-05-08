use std::fs::{self};
use anyhow::Result;
use csv::Reader;
use crate::Player;

/**
 * 数据处理模块
 */

pub fn process_csv(input: &str, output: &str)-> Result<()>{

    // 读取输入文件，解析为对应的实体列表
    let players = Reader::from_path(input)?
        .deserialize()
        .map(|item|item.unwrap())
        .collect::<Vec<Player>>();

    // 转换为JSON格式，写入到输出文件
    let json = serde_json::to_string_pretty(&players)?;
    fs::write(output, json)?;
    Ok(())
}


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
