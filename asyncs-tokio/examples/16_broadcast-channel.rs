use std::time::Duration;

use tokio::sync::broadcast::error::RecvError;

///   `tokio::sync::broadcast` 是一种广播通道，可以有多个发送者和多个接收者，并且发送的每一条消息都能被
/// 所有接收者接收到。
/// 



#[tokio::main]
async fn main(){
    // 创建一个广播通道
    let (tx, mut rx1) = tokio::sync::broadcast::channel::<String>(32);
    let mut tasks = tokio::task::JoinSet::new();

    // 接收者1号
    tasks.spawn(async move {
        loop {
            match rx1.recv().await {
                Ok(msg) => println!("recv1: {}", msg),
                Err(RecvError::Closed) => {
                    // 所有发送者关闭
                    println!("all sender closed => recv1 quit.");
                    return;
                },
                Err(RecvError::Lagged(_)) => println!("lagged.")
            }     
        }
    });

    // 接收者2号
    let mut rx2 = tx.subscribe();
    tasks.spawn(async move {
        loop {
            match rx2.recv().await {
                Ok(msg) => println!("recv2: {}", msg),
                Err(RecvError::Closed) => {
                    println!("all sender closed => recv2 quit.");
                    return;
                },
                Err(RecvError::Lagged(_)) => println!("lagged.")
            }     
        }
    });

    // 发送者1号
    let tx2 = tx.clone();
    tasks.spawn(async move {
        for i in 1..11 {
            tx.send(format!("send1的第 {} 条消息", i)).unwrap();
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    });

    // 发送者2号
    tasks.spawn(async move {
        for i in 1..11 {
            tx2.send(format!("send2的第 {} 条消息", i)).unwrap();
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });


    tasks.join_all().await;
}