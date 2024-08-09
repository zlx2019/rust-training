use std::{ sync::mpsc, thread, time::Duration};
/// 多生产者 - 单消费者
/// 这是 Rust 标准库默认提供的特性


fn main(){
    let (tx, rx) = mpsc::channel();
    // 由于多个生产者都需要 channel的发送者所有权，所以需要克隆多份
    let prod1 = tx.clone();
    let prod2 = tx.clone();
    
    // 生产者1
    thread::spawn(move || {
        for i in 0..10 {
            prod1.send(format!("Prod-1: {}",i)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });
    // 生产者2
    thread::spawn(move || {
        for i in 0..10 {
            prod2.send(format!("Prod-2: {}",i)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    // 生产者3
    thread::spawn(move||{
        for i in 0..10{
            tx.send(format!("Prod-3: {}",i)).unwrap();
            thread::sleep(Duration::from_millis(800));
        }
    });

    // 消费者，直接通过 for 循环来遍历通道，通道关闭后则结束
    // 注意: 所有的发送者都 drop 后通道才会关闭.
    for msg in rx {
        println!("received: {}", msg);
    }

    println!("Program finished")
}