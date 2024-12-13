#![allow(unused_variables)]
#![allow(dead_code)]
use std::time::Duration;

/// `mpsc` 通道可以多个发送者，但是只能有一个接收者。并且 mpsc 分为两种:
///     - bounded channel: 有界通道，可以容纳指定数量消息的通道（数量至少为1）。
///         通过 `mpsc::channel()` 创建.
///     - unbounded channel: 无界通道，通道的容量不上限，可以无限制地存放消息，直到内存用尽。
///         通过 `mpsc::unbounded_channel()`

/// # 关于Sender 和 Receiver 的一些方法说明:
///
/// ## Sender 方法
///     - capacity(): 获取当前通道的可用容量。
///     - closed(): 阻塞等待通道的所有 Receiver 关闭, 当所有 Receiver 关闭后解除阻塞。
///     - is_closed(): 判断 Receiver 端是否已经全部关闭。
///     - send(): 向通道中发送消息，通道已满时会阻塞等待，如果 Receiver 已关闭则会返回Err(e)
///     - send_timeout(): 向通道中发送消息，阻塞指定的时长后返回错误。
///     - try_send(): 尝试向通道中发送数据，如果通道已满不会阻塞，而是立即返回错误。
///     - reserve(): 等待并申请一个通道中的空闲位置，返回一个Permit，申请的空闲位置被占位，且该位置只留给该Permit实例，
///             之后该Permit可以直接向通道中发送消息，并释放其占位的位置。申请成功时，通道空闲容量减1，释放位置时，通道容量会加1。
///     - try_reserve(): 尝试申请一个空闲位置且不等待，如果无法申请，则返回错误。
///     - reserve_owned(): 与reserve()类似，它返回OwnedPermit，但会Move Sender
///     - try_reserve_owned(): reserve_owned()的不等待版本，尝试申请空闲位置失败时会立即返回错误。
///     - blocking_send(): Sender可以在同步代码环境中使用该方法向异步环境发送消息
///
/// ## Receiver 方法
///     - close(): 关闭当前 Receiver。
///     - recv(): 接收消息，如果通道已空，则等待，如果对端已全部关闭，则返回None
///     - try_recv(): 尝试接收消息，没有消息不会阻塞，而是立即返回错误。
///     - blocking_recv(): Receiver可以在同步代码环境中使用该方法接收来自异步环境的消息。
///
/// 需要特别注意的是，Receiver端调用close()方法关闭通道后，只是半关闭状态，Receiver端仍然可以继续读取可能已经缓冲在通道中的消息，
/// close()只能保证Sender端无法再发送普通的消息，但Permit或OwnedPermit仍然可以向通道发送消息。
/// 只有通道已空且所有Sender端(包括Permit和OwnedPermit)都已经关闭的情况下，recv()才会返回None，此时代表通道完全关闭。
///
///   Receiver的try_recv()方法在无法立即接收消息时会立即返回错误。返回的错误分为两种:
///     - TryRecvError::Empty 表示通道没有可读数据，但还有Sender存在。
///     - TryRecvError::Disconnected 表示通道已空，且Sender端(包括Permit和OwnedPermit)已经全部关闭。


///
/// `mpsc` 有界通道 示例1
/// 主任务消费消息，子任务发送10个消息。
async fn example1(){
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);
    tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                println!("receiver closed.");
                return;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        // 延缓3秒后显式关闭发送端
        tokio::time::sleep(Duration::from_secs(3)).await;
        drop(tx);
    });
    // 循环读取队列消息
    loop {
        match rx.recv().await {
            Some(v) => println!("receiver: {}", v),
            None => {
                // 发送者被关闭
                println!("sender closed.");
                break;
            }
        }
    }
    println!("main end");
}

/// 有界队列 示例2
/// 将上面的示例改一下，生成10个异步任务分别发送数据：
async fn example2(){
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(10);
    for i in 1..10 {
        let tx = tx.clone();
        tokio::spawn(async move {
            if tx.send(i).await.is_err() {
                println!("receiver closed.");
            }
        });
    }
    drop(tx);
    while let Some(v) = rx.recv().await {
        println!("receiver: {}", v);
    }
}

/// 关于 reserve 的示例
async fn example3(){
    // 创建容量为1的通道
    let (tx,mut rx) = tokio::sync::mpsc::channel::<()>(1);
    // 申请并占有唯一的空闲位置
    let permit = tx.reserve().await.unwrap();
    // 由于唯一的控制已经被占有，所以此时发送消息则会失败
    assert!(tx.try_send(()).is_err());
    // 通过预占的 permit 发送消息
    permit.send(());
    // 接收消息
    assert_eq!(rx.recv().await.unwrap(), ());

    // 创建容量为1的通道
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    // tx.reserve_owned()会消费掉tx
    let permit = tx.reserve_owned().await.unwrap();
    // 通过permit.send()发送消息，它又返回一个Sender
    let tx = permit.send(456);
    assert_eq!(rx.recv().await.unwrap(), 456);
    //可以继续使用返回的Sender发送消息
    tx.send(789).await.unwrap();
    assert_eq!(rx.recv().await.unwrap(), 789);
}

#[tokio::main]
async fn main(){
    // example1().await;
    // example2().await;
    example3().await;
}