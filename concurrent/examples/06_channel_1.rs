use std::{sync::mpsc, thread, time::Duration};

/// Do not communicate by sharing memory; instead, share memory by communicating
/// 通过 消息传递 进行线程间通信.

/// 单生产 --》 单消费

fn main(){
    // 创建消息通道，返回一个元组(发送者,接收者)
    let (tx, rx) = mpsc::channel();
    // 创建生产者线程
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("producer finished");
    });

    // 主线程负责消费(阻塞式)
    // while let Ok(val) = rx.recv() {
    //     println!("main thread receive: {}", val);   
    // }

    // 非阻塞式
    loop {
        thread::sleep(Duration::from_millis(500));
        match rx.try_recv() {
            Ok(val) => {
                println!("main thread receive: {}", val);
            },
            Err(mpsc::TryRecvError::Empty) => {
                println!("没有读取到数据...");
            },
            Err(mpsc::TryRecvError::Disconnected) =>{
                println!("channel closed...");
                break;
            }
        }
    }
}