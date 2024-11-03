use std::time::Duration;
use futures::executor::block_on;


/// 当在一个异步函数中调用另一个异步任务时, 可以有两种方案:
///  - 使用.await
///  - 对 Future 进行 轮询(poll)
/// 注意，使用.await 并不会阻塞当前线程，而是等待该 Future 执行完成，在此期间

async fn hello_world(){
    std::thread::sleep(Duration::from_secs(3)); // 睡眠3s
    println!("Hello World");
    hello_cat().await;
}

async fn hello_cat(){
    std::thread::sleep(Duration::from_secs(3)); // 睡眠5s
    println!("Hello kitty");
}

fn main(){
    block_on(hello_world());
}