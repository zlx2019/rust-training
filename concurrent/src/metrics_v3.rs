#![allow(dead_code)]

/// 
/// 使用第三方库 dashmap 优化
/// 

use std::fmt;
use std::sync::Arc;
use dashmap::DashMap;

/// 信息统计容器
#[derive(Debug, Clone)]
pub struct Metrics {
    /// 通过Mutex保证并发安全，通过Arc实现Mutex在多线程环境下的使用
    data: Arc<DashMap<String, i64>>
}
impl Metrics{
    pub fn new() -> Self{
        Metrics{data: Arc::new(DashMap::new())}
    }
    /// 统计指标递增
    pub fn incr(&self, key: impl Into<String>) -> anyhow::Result<()>{
        let mut counter= self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
    /// 统计指标递减
    pub fn decr(&self, key: impl Into<String>) -> anyhow::Result<()>{
        // 加锁
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }
}


impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter(){
            writeln!(f, "{} : {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}