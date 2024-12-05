use std::time::Duration;

use tokio::task::JoinSet;

/// # JoinSet
/// 可以使用 `tokio::task::JoinSet` 来收集一批异步任务，可以用于控制所有任务结束或其中任意一个任务终止。

#[tokio::main]
async fn main(){
    let mut tasks: JoinSet<()> = tokio::task::JoinSet::new();
    // 创建10个异步任务
    for i in 0..10 {
        tasks.spawn(async move{
            tokio::time::sleep(Duration::from_secs(i)).await;
            println!("task {} finished.", i);
        });
    }
    // 等待其中任意一个任务完成
    // tasks.join_next().await;
    // 终止其他所有任务
    // tasks.abort_all();
}