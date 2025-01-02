use std::time::Duration;

/// watch 通道只能有一个 Sender，但是可以拥有多个 Recviver,并且通道中只能存在一个消息
/// 对于 Sender 来说每次发送消息都是修改通道中存在的那个唯一数据。
/// 对于 Receiver 来说消费消息其实就是监听消息的改变。


#[tokio::main]
async fn main(){
    // 创建 watch 通道，并且初始化消息
    let (tx, mut rx) = tokio::sync::watch::channel::<usize>(0);

    // 接收者(监听者)
    let observer_task = tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            println!("msg change: {}", *rx.borrow());
        }
    });

    // 发送者
    let sender_task = tokio::spawn(async move {
        for i in 1..11 {
            tx.send(i).unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    _ = tokio::join!(observer_task, sender_task);
}