use std::{thread, time::Duration};

/// 创建线程
fn main(){
    // 创建子线程
    let task = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_secs(1));
        }
    });
    task.join().unwrap();
    // 主线程
    for i in 1..5{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_secs(1));
    }
}