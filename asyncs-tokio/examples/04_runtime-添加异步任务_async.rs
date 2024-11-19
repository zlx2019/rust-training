use std::time::Duration;
use chrono::Local;

/// runtime 的 `block_on` 是添加异步任务的主要方式，但是它会使得当前线程阻塞。
///   所以还有另一种方式: 使用 runtime 的 `enter`, 并且它不会阻塞当前线程，它会返回一个EnterGuard,
/// 它没有其他作用，仅仅表示在它之后声明的所有异步任务，都会在 runtime 上下文中执行，直到将它 drop。
fn main(){
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut tasks = tokio::task::JoinSet::new();
    // 进入 runtime 上下文
    let ctx1 = runtime.enter();
    // 创建异步任务
    // 生成的 异步任务 会自动添加到 runtime 上下文
    tasks.spawn(async {
        tokio::time::sleep(Duration::from_secs(5)).await;
        println!("task1 sleep over: {}", now());
    });
    tasks.spawn(async {
        tokio::time::sleep(Duration::from_secs(7)).await;
        println!("task2 sleep over: {}", now());
    });
    // 退出上下文
    drop(ctx1);

    // 可以再次进入上下文，并且添加异步任务
    let  ctx2 = runtime.enter();
    tasks.spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("task3 sleep over: {}", now());
    });
    tasks.spawn(async {
        tokio::time::sleep(Duration::from_secs(9)).await;
        println!("task4 sleep over: {}", now());
    });
    drop(ctx2);
    // 等待所有异步任务结束.
    runtime.block_on(tasks.join_all());
    println!("program over runtime exit: {}", now());
}


fn now() -> String {
    Local::now().format("%F %T").to_string()
}