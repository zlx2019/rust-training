use std::time::Duration;

use chrono::Local;


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

fn main(){
    wait_all_task();
    wait_all_task_or_top_err();
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}