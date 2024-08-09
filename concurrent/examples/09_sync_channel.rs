use std::{sync::mpsc, thread, time::Duration};

/// 同步和异步通道
/// Rust标准库的 `mpsc` 通道分为两种类型：同步和异步.
/// 上几个案例中都是使用的异步通道: 
///     - 异步: 无论是否有接收者正在等待消息，消息发送者发送时都不会阻塞。
///     - 同步: 只有接受者拿到消息后，发送者才会接触阻塞。
/// 下面演示同步通道的使用.


fn main(){
    // 创建一个同步通道
    let (tx, rx) = mpsc::sync_channel::<i32>(0);
    
    // 发送者发送10条消息(不睡眠)
    thread::spawn(move|| {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
        println!("sender finished.");
    });

    // 消费者每隔1秒 消费一次.
    for m in rx{
        println!("{}", m);
        thread::sleep(Duration::from_secs(1));
    }
    println!("progarm finsished.")
}

// 通过 `mpsc::channel()` 创建的是异步通道.
// 
// 通过 `mpsc::sync_channel(n)` 创建的是同步通道，而参数 n 则表示了同步通道中的缓存大小
// 当你设定为N时，发送者就可以无阻塞的往通道中发送N条消息，当消息缓冲队列满了后，新的消息发送将被阻塞(如果没有接收者消费缓冲队列中的消息，那么第N+1条消息就将触发发送阻塞)。