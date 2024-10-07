#![allow(dead_code)]

/// 使用读写锁优化 metrics

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc,RwLock};
use anyhow::anyhow;

/// 信息统计容器
#[derive(Debug, Clone)]
pub struct Metrics {
    /// 通过Mutex保证并发安全，通过Arc实现Mutex在多线程环境下的使用
    data: Arc<RwLock<HashMap<String, i64>>>
}
impl Metrics{
    pub fn new() -> Self{
        Metrics{data: Arc::new(RwLock::new(HashMap::new()))}
    }
    /// 统计指标递增
    pub fn incr(&self, key: impl Into<String>) -> anyhow::Result<()>{
        // 加锁
        let mut data = self.data.write()
            .map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    /// 统计指标递减
    pub fn decr(&self, key: impl Into<String>) -> anyhow::Result<()>{
        // 加锁
        let mut data = self.data.write()
            .map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
    /// 获取当前统计信息
    pub fn snapshot(&self) -> anyhow::Result<HashMap<String, i64>>{
        let data = self.data.read()
            .map_err(|e| anyhow!(e.to_string()))?;
        Ok(data.clone())
    }
}


impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.read().unwrap();
        for (key, value) in data.iter(){
            writeln!(f, "{} : {}", key, value)?;
        }
        Ok(())
    }
}