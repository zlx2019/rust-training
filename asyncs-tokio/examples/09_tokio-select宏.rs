#![allow(dead_code)]
#![allow(unused_variables)]
use std::time::Duration;

use tokio::signal;

/// 在Golang中有一个select关键字，用于以非阻塞式读取channel中的数据。tokio中则类似提供了一个名为`tokio::select!`的宏。
/// select! 用于轮询多个异步任务, 当有任务结束则执行其分支。

/// 示例如下
#[tokio::main]
async fn main(){
    // 示例一: 处理最先执行完成的异步任务
    tokio::select! {
        value = sleep(5) => println!("branch 1 task done: {}", value),
        value = sleep(7) => println!("branch 2 task done: {}", value),
    };
    println!("main thread done.");


    // 示例二: 阻塞执行一个异步任务，带有超时时间
    let async_task = async{
        tokio::time::sleep(Duration::from_secs(10)).await;
        "Task Result"
    };
    tokio::select! {
        val = async_task => println!("task1 over: {}", val),
        // 任务3秒未结束则超时
        _ = sleep(3) => println!("task timeout."),
    }

    
    // 示例三: 执行任务，通过信号控制超时
    tokio::select! {
        _ = sleep(10) => println!("task2 over"),
        _ = signal::ctrl_c() => println!("task signal interrupt")
    }
}

async fn sleep(n: u64) -> u64{
    tokio::time::sleep(Duration::from_secs(n)).await;    
    n
}