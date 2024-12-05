#![allow(dead_code)]
#![allow(unused_variables)]

use std::{thread, time::Duration};

use chrono::Local;
use tokio::runtime;

///   在 tokio 中有两种线程
///     - 用于异步任务的工作线程 (Worker Thread): 
///     - 用于同步任务的阻塞线程 (Blocking Thread)
///   单线程或者多线程模式的 runtime 指的都是异步工作线程，即只用于执行异步任务的线程，这些任务主要是IO密集型任务
/// tokio 默认会将每一个工作线程绑定到每一个CPU核心上。
/// 
///   但是，有些必要的任务可能会长时间计算而占用线程，甚至任务可能是同步的，它会直接阻塞整个线程(比如thread::time::sleep())，
/// 这类任务如果计算时间或阻塞时间较短，勉强可以考虑留在异步队列中，但如果任务计算时间或阻塞时间可能会较长，它们将不适合放在异步队列中，
/// 因为它们会破坏异步调度，使得同线程中的其它异步任务处于长时间等待状态，也就是说，这些异步任务可能会被饿很长一段时间。

async fn blocking_task(){
    // 例如 直接在异步任务中执行一些阻塞工作线程的操作,由于这类阻塞操作不在 tokio 系统内，tokio无法识别这类线程阻塞的操作，
    // tokio只能等待该线程阻塞操作的结束，才能重新获得那个线程的管理权    
    std::thread::sleep(Duration::from_secs(10));
    
    // blocking thread默认是不存在的，只有在调用了spawn_blocking()时才会创建一个对应的blocking thread。
    tokio::task::spawn_blocking(||{
        std::thread::sleep(Duration::from_secs(5));
    });
}


/// blocking thread 通常不用于执行异步任务，因此 runtime 并不会调度这类线程，它们本质上相当于一个独立的 thread::spawn() 创建的线程.
/// 它也不会像 block_on 一样会阻塞当前线程, 它和独立线程的唯一区别就是它是在 runtime 内的，可以在runtime中进行一些异步操作.
fn main(){
    let rt = runtime::Runtime::new().unwrap();
    // 使用阻塞线程运行任务
    let blocking_task = rt.spawn_blocking(|| {
        println!("begin blocking task {}", now());
        thread::sleep(Duration::from_secs(5));
    });

    thread::sleep(Duration::from_secs(2));
    // 在异步任务内 等待 阻塞线程任务结束.
    rt.block_on(async {
        blocking_task.await.unwrap();
        println!("end blocking task {}", now());
    });
}
// 注意: blocking thread 任务虽然是在 runtime 中,但是不受 tokio 调度控制.
// 因此，如果在block_on()中生成了blocking thread或普通的线程，block_on()不会等待这些线程的完成。

fn now() -> String {
    Local::now().format("%F %T").to_string()
}