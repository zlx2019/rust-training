use std::sync::Mutex;


/// 互斥锁 Mutex
/// 使得多个线程访问同一个资源时，串行化.

fn main(){
    // 创建共享资源，由Mutex保护
    let year: Mutex<i32> = Mutex::new(1999);
    {
        let mut y = year.lock().unwrap(); // 获取锁
        *y = 2024 // 修改资源
    }// drop 释放锁

    println!("year = {:?}", year);
}