use std::{sync::{Arc, Barrier}, thread};


/// 线程屏障
/// 使用 `Barrier` 可以使多个线程执行到同一个点后阻塞，然后一起继续执行.
fn main(){
    let mut handlers = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handlers.push(thread::spawn(move ||{
            println!("before wait");
            b.wait();// 等待
            println!("after wait");
        }));
    }

    for handle in handlers{
        handle.join().unwrap();
    }
}