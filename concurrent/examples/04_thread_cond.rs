use std::{sync::{Arc, Condvar, Mutex}, thread, time::Duration};


/// 使用 mutex 搭配 condvar 实现线程的挂起与唤醒

fn main(){
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move|| {
        thread::sleep(Duration::from_secs(5));
        let (lock, cvar) = &*pair2; // 解Arc，获取 mutex 和 condver
        let mut started = lock.lock().unwrap(); // 加锁
        println!("other thread changing started");
        *started = true;
        cvar.notify_one(); // 唤醒主线程
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        // 阻塞等待唤醒
        started = cvar.wait(started).unwrap();
    }
    println!("started changed main thread stop.");
}