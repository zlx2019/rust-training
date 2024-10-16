use std::sync::{Arc, Mutex};
use std::thread;

/// 多线程环境下，使用 `Mutex`
/// 单线程中使用锁，毫无用武之地，有它没它没差别
/// 毕竟多线程才会使用到锁，下面使用 `Mutex`在多线程环境下，访问同一资源
/// 多线程共享资源，需要使用到`Arc`,而不是`Rc`。
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut tasks= vec![];

    // 开启10个线程
    for _ in 0..10 {
        let counter =  Arc::clone(&counter);
        let task = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        tasks.push(task);
    }
    
    // 等待所有线程结束
    for task in tasks {
        task.join().unwrap();
    }

    println!("Result :{}", *counter.lock().unwrap())
}