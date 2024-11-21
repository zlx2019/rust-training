#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Duration;
use chrono::Local;

/// 什么是Task？
///   官方手册中使用一句话介绍Task: Asynchronous green-threads(异步的绿色线程)。也就是所谓的协程概念，也即一个可临时中断和恢复的函数。
/// 
/// - Rust的原生线程(`std::thread`) 是OS线程，也就是操作系统线程。由操作系统内核负责管理和调度，所以会涉及到频繁的系统调用，它的上下文切换开销很大。
/// - [协程|用户态线程|绿色线程] 则由程序层面实现的调度器负责调度，所以不涉及系统调用，同一个OS线程内的多个协程的上下文切换的开销是非常小的，因此非常轻量级。
/// 
/// 那么tokio的task是什么呢？
///   在Rust中每定义一个 `Future`时, 就定义了一个静止尚未执行的 task, 当它在 runtime 中开始运行的时候，它就是一个真正的task: 异步任务。
///   例如:
///     - async{}
///     - async fn task(){}
///   这些都是一个Future，它们被编译之后都是一个实现了 Future 特征的结构体.
/// 
///   要注意，在tokio runtime中执行的并不都是异步任务，绑定在runtime中的可能是同步任务(例如一个数值计算就是一个同步任务，只是速度非常快，可忽略不计)，
/// 可能会长时间计算，可能会阻塞整个线程，这一点在前一篇介绍runtime时详细说明过。tokio严格区分异步任务和同步任务，只有异步任务才算是tokio task。
/// tokio推荐的做法是将同步任务放入blocking thread中运行。
/// 
/// `tokio::task`模块提供了下列几个函数: 
///     - spawn: 向runtime中添加异步任务。
///     - spawn_blocking: 创建一个阻塞线程执行指定的同步任务。
///     - block_in_place: 创建一个同步任务并且在异步工作线程中执行(会把这个异步线程中的所有异步任务转移走，防止饿死)。
///     - yield_now: 使当前任务放弃CPU执行权，与线程解绑并且进入就绪队列等待下一轮调度。
///     - unconstrained: 创建一个不受调度器调度的异步任务，也就是说该任务不会被调度器轻易的被抢占式策略打断，
///     会尽量一直霸占着执行权直到结束(可能在任何工作线程上)。适合一些长时间CPU计算密集型任务。
///     - spawn_local: 创建一个不允许被窃取的异步任务。该任务只会在一个固定的工作线程上执行，不会被其他工作线程窃取走。

/// # 异步任务返回类型
/// 
/// 以 spawn 开头的方法都会返回 JoinHandle 类型，该类型可以通过 await 来等待异步任务完成
/// 也可以通过 abort()方法中断异步任务，被中断后返回 JoinError 类型
async fn spawn_fn(){
    // 阻塞等待异步任务完成
    let task_handle1 = tokio::task::spawn(async{});
    task_handle1.await.unwrap();

    // 中断异步任务
    let task_handle2 = tokio::task::spawn(async{});
    match task_handle2.await {
        Ok(value) => println!("task2 over!"),
        Err(err) => println!("task2 abort!"),
    }
}


/// # task::block_in_place
/// 
/// `block_in_place`方法的目的和`spawn_blocking`方法类似，都是用于执行同步任务，区别是:
///     - spawn_blocking会创建一个新的阻塞线程来执行该同步任务。
///     - 而block_in_place是在当前工作线程执行该同步任务，但在执行前会把当前工作线程中的所有异步任务转移给其他工作线程。
/// 所以显然，block_in_place只应该在多线程模式的 runtime 中运行，如果是单线程的则会阻塞唯一的工作线程。
async fn block_in_place_fn(){
    tokio::task::block_in_place(move || {
        // 做一些计算量大的工作或调用同步代码...
        std::thread::sleep(Duration::from_secs(10));
    });
    // 在block_in_place 内部可以重新进入 runtime 执行一些异步任务.
    tokio::task::block_in_place(move || {
        tokio::runtime::Handle::current().block_on(async move {
            // 做一些异步操作...
        });
    });
}

/// # task::yield_now
/// 
/// 当前任务立即放弃CPU, 将工作线程归还给调度器，任务自身进入就绪任务队列等待下次被调度。
/// 注意，yield_now 本身也是一个异步任务，需要await。
/// 注意，yield后，任务调度的顺序是未知的。有可能任务在发出yield后，紧跟着的下一轮调度会再次调度该任务。
async fn yield_now_fn(){
    async {
        tokio::task::spawn(async{
            println!("spawned task done!");
        });
        // 尝试让出执行权
        tokio::task::yield_now().await;
        println!("main task done!");
    }.await;
}

/// # task::abort
/// 
/// 正在执行的task可以随时被abort方法所中断取消，取消之后任务返回的Err(JoinError)类型
/// 如果异步任务已完成，再对该任务执行 abort 操作将没有任何效果
fn cancel_task(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async{
        // 开始一个异步任务
        let task = tokio::task::spawn(async{
            println!("begin task {}", now());
            tokio::time::sleep(Duration::from_secs(5)).await;
        });
        // 2秒后 中断这个异步任务
        tokio::time::sleep(Duration::from_secs(2)).await;
        task.abort();
        match task.await {
            Ok(_) => todo!(),
            Err(err) => println!("任务被中断结束：{}, {}", err.is_cancelled(), now()),
        }
    });
}


/// # 通过宏来等待一个或多个异步任务完成
/// 
/// 通过 .await 可以等待某个异步任务的完成，无论这个任务是正常完成还是被 abort 取消.
/// 除此之外, tokio 还提供了两个宏来等待多个异步任务的完成
///     - `tokio::join!`: 等待所有的异步任务完成，将所有异步任务的返回值以元组方式返回。
///     - `try_join!`: 等待所有异步任务完成，一旦有一个任务返回Error时，就会立即返回一个Error，返回值为 Result<(()...), Error>

/// 等待多个异步任务全部结束
fn wait_all_task(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async{
        let task_one = async {
            println!("begin one: {}", now());
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("end one: {}", now());
            1
        };
        let task_two = async {
            println!("begin two: {}", now());
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("end two: {}", now());
            2
        };
        // 等待两个异步任务完成
        let res = tokio::join!(task_one, task_two);
        println!("all task over: {:?}", res);
    });
}

/// 等待多个异步任务完成，有任务失败则获取最早失败任务的Error
async fn do_task_one() -> Result<i32, &'static str>{
    tokio::time::sleep(Duration::from_secs(3)).await;
    Ok(1001)
}
async fn do_task_two() -> Result<i32, &'static str>{
    tokio::time::sleep(Duration::from_secs(5)).await;
    // Err("task2 error")
    Ok(1002)
}
fn wait_all_task_or_top_err(){
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async{
        let res=  tokio::try_join!(do_task_one(), do_task_two());
        match res {
            Ok(value) => println!("所有任务完成, 结果集: {:?}", value),
            Err(err) => println!("有一个任务产生错误: {}", err),
        }
    });
}

fn main (){
    // wait_all_task();
    wait_all_task_or_top_err();
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}