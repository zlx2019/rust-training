#![allow(dead_code)]
#![allow(unused_variables)]

use std::{future::Future, sync::{Arc, Mutex}, task::{Poll, Waker}, thread::{self}, time::Duration};

use chrono::Local;

/// 借助于 Tokio 运行时，实现一个 PauseFuture 异步任务：使任务阻塞一定的时长。
/// 


#[tokio::main]
async fn main(){
    println!("{} main begin. ", now());
    PauseFuture::new(Duration::from_secs(5)).await;
    println!("{} main end. ", now());
}

/// 异步任务状态
struct State{
    // 唤醒函数，由异步运行时调用Future#poll方法时传入
    // 当任务从某个阻塞状态结束后，回调该函数，通知运行时任务已完成.
    waker: Option<Waker>,
    inner_state: PauseState
}

/// PauseFuture 任务的内部状态
/// 表示Future是初始化、睡眠状态或已结束。
#[derive(PartialEq)]
enum PauseState {
    Init,       // 初始状态
    Sleeping,   // 睡眠中
    Done        // 已完成
}


/// Pause 这是一个实现了 Future 特征的结构体.
/// 该Future 会阻塞一定的时长: 其内部开启一个新的线程来睡眠，睡眠结束后任务则结束
struct PauseFuture{
    // 休眠时长
    pause_time: Duration,
    // Future状态
    state: Arc<Mutex<State>>,
}

impl PauseFuture {
    // 构建函数
    fn new(pause_time: Duration) -> Self {
        PauseFuture{ 
            pause_time,
            state: Arc::new(Mutex::new(State{ waker: None, inner_state: PauseState::Init }))
        }
    }
}

/// 为 PauseFuture 实现 Future 特征
/// 只有实现了Future 才可以作为一个异步的Task
impl Future for PauseFuture{
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // 获取任务状态的锁
        let mut guard = self.state.lock().unwrap();
        match guard.inner_state {
            // 任务已结束
            PauseState::Done => Poll::Ready(()),
            // 任务还未开始, 开始运行任务
            PauseState::Init => {
                // 保存waker函数
                guard.waker = Some(ctx.waker().clone());
                // 将任务状态推进都按运行中
                guard.inner_state = PauseState::Sleeping;
                // 开启线程运行任务，将状态和休眠时长 move 到线程内
                let state_cloned = Arc::clone(&self.state);
                let pause_time = self.pause_time;
                thread::spawn(move ||{
                    println!("{} task start.", now());
                    thread::sleep(pause_time);
                    // 睡眠结束，表示任务完成
                    // 将状态更新为已完成,回调waker
                    let mut guard = state_cloned.lock().unwrap();
                    guard.inner_state = PauseState::Done;
                    if let Some(waker) = guard.waker.take(){
                        // 回调运行时，否则任务则不会被再次调度
                        waker.wake();
                    }
                    println!("{} task closed.", now());
                });
                Poll::Pending
            },

            // 任务运行中...
            PauseState::Sleeping => Poll::Pending,
        }
    }
}



fn now() -> String {
    Local::now().format("%F %T").to_string()
}