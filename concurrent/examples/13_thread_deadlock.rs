use std::sync::Mutex;

/// mutex 使用不当很容易产生死锁

fn main(){
    let m = Mutex::new(1999);

    // 获取锁
    let mut year = m.lock().unwrap();
    *year = 2024;

    // 再次获取锁，上次还未释放
    year = m.lock().unwrap();

    // 永久阻塞...
}