#![allow(dead_code)]
#![allow(unused_variables)]

/// 什么是Task？
///   官方手册中使用一句话介绍Task: Asynchronous green-threads(异步的绿色线程)。也就是所谓的协程概念，也即一个可临时中断和恢复的函数。
/// 
/// - Rust的原生线程(`std::thread`) 是OS线程，也就是操作系统线程。由操作系统内核负责管理和调度，所以会涉及到频繁的系统调用，它的上下文切换开销很大。
/// - [协程|用户态线程|绿色线程] 则由程序层面实现的调度器负责调度，所以不涉及系统调用，同一个OS线程内的多个协程的上下文切换的开销是非常小的，因此非常轻量级。
/// 
/// 那么tokio的task是什么呢？
///   在Rust中每定义一个 `Future`时, 就定义了一个静止尚未执行的 task, 当它在 runtime 中开始运行的时候，它就是一个真正的task: 异步任务。
/// 
///   要注意，在tokio runtime中执行的并不都是异步任务，绑定在runtime中的可能是同步任务(例如一个数值计算就是一个同步任务，只是速度非常快，可忽略不计)，
/// 可能会长时间计算，可能会阻塞整个线程，这一点在前一篇介绍runtime时详细说明过。tokio严格区分异步任务和同步任务，只有异步任务才算是tokio task。
/// tokio推荐的做法是将同步任务放入blocking thread中运行。
/// 
/// 
/// 

// 这是一个 Future
async fn async_task(){
    // 这也是一个 Future
    let _task = async{};
}




fn main (){
    
}