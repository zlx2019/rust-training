use chrono::Local;
use std::time::Duration;
use tokio::runtime;

/// runtime 执行 task.
/// 多数情况下，异步任务(task)都是一些带网络IO操作的任务，比如异步的http请求等，比如 tokio::time::sleep()
/// std::thread::sleep() 提供的方法会阻塞整个工作线程，
/// tokio::time::sleep() 会使当前异步任务与工作线程解绑，让出执行权，然后进入任务调度队列等待被唤醒，它不会阻塞工作线程，而是让工作线程去执行其他异步任务。
fn main(){
    let rt = runtime::Runtime::new().unwrap();
    // 创建异步任务 (异步代码块)
    let task = async {
        println!("before sleep: {}",Local::now().format("%F %T"));
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("after sleep: {}",Local::now().format("%F %T"));
        1024
    };
    // 将异步任务添加到运行时，并且阻塞等待该任务结束.
    // block_on() 会使当前线程阻塞.
    let task_result = rt.block_on(task);
    println!("task result: {:?}", task_result);

    // 添加异步任务，在异步任务中执行其他异步任务
    rt.block_on(async {
        create_async_task().await.unwrap();
    });
}

fn create_async_task() -> tokio::task::JoinHandle<()>{
    println!("create an async task: {}", now());
    // 创建一个新的异步任务
    tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(10)).await;
        println!("async task over: {}", now());
    })
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}