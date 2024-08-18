#![allow(dead_code)]  // 忽略本文件的一些提示

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};


/**
 *  Player 实体
 */
#[allow(dead_code)]
#[derive(Debug,Deserialize,Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player{
    pub name: String,           // 名称
    pub position: String,        // 队形位置
    #[serde(rename = "DOB")]
    pub dob: String,            // 出生日期
    pub nationality: String,    // 国籍
    #[serde(rename = "Kit Number")]
    pub number: u8              // 球衣号码
}


/// Player 方法
impl Player {
    // JSON 序列化
    pub fn to_json(&self) -> Result<String>{
        Ok(serde_json::to_string(&self)?)
    }
    // JSON 反序列化
    pub fn from_json(json: &str) -> Result<Self>{
        Ok(serde_json::from_str(json)?)
    }
}
