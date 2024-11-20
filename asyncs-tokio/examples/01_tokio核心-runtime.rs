#![allow(dead_code)]
#![allow(unused_variables)]

use std::{thread, time::Duration};

use tokio::runtime::{self};

/// 在 Tokio 中, 存在两个非常重要的概念: runtime 和 task。
///  - runtime: 异步运行时环境。
///  - task: runtime中所执行的一个个任务。

fn create_tokio_runtime() {
    // 创建 tokio 运行时, 默认为多线程模式, 工作线程数量和CPU核数一致
    let rt = tokio::runtime::Runtime::new().unwrap();
    // 指定参数
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8) // 工作线程数
        .enable_io() // 启用异步IO
        .enable_time() // 启用异步计时器
        .build()
        .unwrap();
    // 创建单线程 运行时
    tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
}


/// 对于 main 函数, 可以通过 #[tokio::main] 宏使得 main 函数本身成为一个运行时运行时, 默认为多线程模式
/// 多线程模式，指定工作线程数量
/// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
/// 单线程模式
/// #[tokio::main(flavor = "current_thread")]
// #[tokio::main]
fn main() {
}

/// 多个 runtime 共存
/// 可以创建多个 runtime 再各自的线程中独立使用.
fn runtime_shared(){
    let t1 = thread::spawn(|| {
        let singlet_runtime= runtime::Builder::new_current_thread().build().unwrap();
        thread::sleep(Duration::from_secs(10));
    });

    let t2 = thread::spawn(||{
        let multi_runtime = runtime::Builder::new_multi_thread().worker_threads(10).build().unwrap();
        thread::sleep(Duration::from_secs(10));
    });
    t1.join().unwrap();
    t2.join().unwrap();

    // 该函数中，启动了两个 runtime, 分别有2个子线程 + (1 + 10)个 runtime 工作线程. 
    // runtime 实现了 Send 和 Sync 特征，因此可以使用 Arc 在多线程环境下使用.
}