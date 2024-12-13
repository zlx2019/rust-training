#![allow(unused_variables)]
#![allow(dead_code)]

use std::time::Duration;

use chrono::Local;
use tokio::sync::oneshot::{self, error::TryRecvError};


/// oneshot channel的特性是：单发送者单接收者，以及单条消息，简单来说就是一次性通道。
/// 非常适用于一些异步任务同步、通知等场景
/// 
/// 通道分别分别有两种角色：
/// - Sender(发送者)
/// - Receiver(接受者) 

async fn sender_and_receiver(){
    // 创建一个 oneshot 通道，获取发送者和接收者
    let (tx, mut rx) = tokio::sync::oneshot::channel::<i32>();

    // 判断 rx 是否已关闭
    if !tx.is_closed() {
        // 向通道内发送数据
        // 发送成功则返回 Ok(())  失败则返回Err(T)
        match tx.send(1001) {
            Ok(()) => println!("发送成功."),
            Err(val) => println!("消息 {} 发送失败", val),
        }
    }

    // 阻塞式消费数据
    // match rx.await {
    //     Ok(v) => println!("消费到数据： {}", v),
    //     Err(e) => println!("客户端关闭"),
    // }

    // 非阻塞式消费数据
    match rx.try_recv() {
        Ok(v) => println!("消费到数据：{}", v),
        Err(TryRecvError::Empty) => println!("表示当前通道内没有可消费的数据"),
        Err(TryRecvError::Closed) => println!("发送端已closed, 并且没有可消费的数据")
    };
}


/// 示例场景：开启一个子任务，主任务阻塞等待，当子任务完成后通知主任务结果并且退出。
#[tokio::main]
async fn main(){
    println!("主任务启动.");
    let (tx, rx) = oneshot::channel::<bool>();
    
    // 开启子任务执行
    tokio::spawn(async move {
        println("子任务开始运行...");
        tokio::time::sleep(Duration::from_secs(2)).await;
        // 通知主任务执行结果
        match tx.send(true) {
            Ok(()) => println!("通知成功."),
            Err(_) => println!("通知失败，可能接收端已关闭."),
        }
    });

    // 等待子任务通知
    match rx.await {
        Ok(v) => println!("子任务完成，结果：{}", v),
        Err(e) => println!("错误：{}", e),
    }
    println!("主任务结束.")
}


fn println(msg: &str){
    println!("{} {}", now(), msg);
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}