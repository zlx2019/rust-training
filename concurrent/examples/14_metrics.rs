use std::thread;
use std::time::Duration;
use anyhow::{Ok, Result};
use concurrent::metrics::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

/// metrics test unit
fn main() -> Result<()> {
    // 创建指标统计器
    let metrics = Metrics::new();
    for idx in 0..N{
        task_worker(idx, metrics.clone())?;
    }
    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        // 每隔10秒，输出统计信息
        thread::sleep(Duration::from_secs(2));
        println!("{:?}", metrics.snapshot());
    }
}

/// 启动一个线程，动态添加指标信息
fn task_worker(id: usize, metrics: Metrics) -> Result<()>{
    thread::spawn(move ||{
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..3000)));
            metrics.incr(format!("task.thread.worker.{}", id))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}

/// 请求处理指标
fn request_worker(metrics: Metrics) -> Result<()>{
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..1000)));
            let page = rng.gen_range(1..10);
            metrics.incr(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });
    Ok(())
}