use std::{thread, time::Duration};
use futures::executor::block_on;

///   async/.await 是 Rust 内置的语言特性，可以让我们用同步的方式去编写异步的代码。
///   通过 async 标记的语法块会被转换成实现了Future特征的状态机。 与同步调用阻塞当前线程不同，
/// 当Future执行并遇到阻塞时，它会让出当前线程的控制权，这样其它的Future就可以在该线程中运行，这种方式完全不会导致当前线程的阻塞。

fn main(){
    // 调用异步函数，返回一个Future
    let future = do_something();
    // block_on 会阻塞当前线程，直到指定的 Future 执行完成
    block_on(future);
}


/// 定义一个异步函数
async fn do_something(){
    thread::sleep(Duration::from_secs(5)); // 睡眠5s
    println!("Hello");
}