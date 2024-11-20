#![allow(dead_code)]
#![allow(unused_variables)]


use std::{thread, time::Duration};

use chrono::Local;
use tokio::runtime;

/// Runtime 可以通过 drop 进行回收, 这时 Runtime 会被关闭(Shutdown).
fn drop_runtime(){
    let rt = runtime::Runtime::new().unwrap();
    drop(rt); // 手动回收 runtime

    // 关闭 Runtime 后, 那么其中的所有异步任务被移除，流程如下: 
    // 1. 移除任务队列，不再接收也不再调度新的任务。
    // 2. 等待工作线程正在执行的任务结束，并终止工作线程。
    // 3. 移除事件循环
}

/// 注意，这种删除runtime句柄的方式只会立即关闭未被阻塞的worker thread，
/// 那些已经运行起来的blocking thread以及已经阻塞整个线程的worker thread仍然会执行
/// 但是，删除runtime又要等待runtime中的所有异步和非异步任务(会阻塞线程的任务)都完成，因此删除操作会阻塞当前线程。
fn wait_drop_runtime(){
    let rt = runtime::Runtime::new().unwrap();

    // 开启一个阻塞线程任务, 回收 runtime时，不会终止该任务，直到自己终止
    rt.spawn_blocking(||{
        thread::sleep(Duration::from_secs(5));
        println!("blocking task over: {}", now());
    });

    let _guard = rt.enter();

    // 异步任务1: 任务会阻塞3秒，但是由于 runtime 1秒后就回收了，所以导致任务终止.
    rt.spawn(async{
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("async task1 over: {}", now());
    });
    // 异步任务2: 阻塞整个工作线程
    rt.spawn(async{
        thread::sleep(Duration::from_secs(4));
        println!("async task2 over: {}", now());
    });
    drop(_guard);

    thread::sleep(Duration::from_secs(1));

    drop(rt);
    println!("runtime droped: {}", now());
}

/// 
/// tokio提供了另外两个关闭runtime的方式：shutdown_timeout()和shutdown_background()。
/// 前者会等待指定的时间，如果正在超时时间内还未完成关闭，将强行终止runtime中的所有线程。后者是立即强行关闭runtime。
fn main(){
    wait_drop_runtime();
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}