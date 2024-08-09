use std::{io, sync::mpsc, thread, time::Duration};

/// 使用通道来传输数据，一样要遵循 Rust 的所有权规则：
/// - 若值的类型实现了Copy特征，则直接复制一份该值，然后传输过去，例如之前的i32类型
/// - 若值没有实现Copy，则它的所有权会被转移给接收端，在发送端继续使用该值将报错

fn main(){
    let (tx, rx) = mpsc::channel();
    
    // 消费者
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    println!("reveice: {}", msg);
                },
                Err(e)=> {
                    println!("channel closed... {:?}", e);
                    break;
                }
            }
        }
    });

    // 读取标准输入，发送到通道
    for line in io::stdin().lines() {
        tx.send(line.unwrap()).unwrap();
    }
    thread::sleep(Duration::from_secs(1));
}