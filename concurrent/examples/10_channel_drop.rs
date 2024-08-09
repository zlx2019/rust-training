use std::{sync::mpsc, thread, time::Duration};

/// 通道何时会关闭？
/// 在之前的案例中，并没有看到显式的关闭通道，那么该如何关闭通道呢？
/// 其实很简单：所有`发送者` 或 所有`接受者`被 drop 后，通道就会关闭。
/// 并且这是编译期间实现的，没有任何损耗

// 之前案例的关闭都是靠 发送者 drop 后关闭的通道，下面尝试通过 接收者 drop 而关闭通道
fn main(){
    let (tx, rx) =  mpsc::channel::<i32>();

    // 发送者发送10条消息
    thread::spawn(move||{   
        for i in 0..10{
            match tx.send(i + 1) {
                Ok(_)=>{
                },
                Err(e) => {
                    println!("channel closed cannot send: {}", e);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 消费者接收5条消息，然后drop
    for i in 0..5{
        let m = rx.recv().unwrap();
        println!("receive {} msg {}", i + 1, m);
        thread::sleep(Duration::from_secs(1));
    }
    drop(rx); // 显式关闭接受者
    println!("consumer finished.");

    // 阻塞5秒
    thread::sleep(Duration::from_secs(5));
}