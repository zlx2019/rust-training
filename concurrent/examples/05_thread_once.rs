use std::{sync::Once, thread};

/// 通过 `Once` 保证一个函数只会执行一次，类似于 Go中的 `sync.Once`

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main(){
    // 启动两个线程修改 VAL 值，但只会有一个线程执行
    let t1 = thread::spawn(||{ 
        INIT.call_once(||{
            unsafe{
                VAL = 1;
            }
        })
    });

    let t2 = thread::spawn(||{
        INIT.call_once(||{
            unsafe{
                VAL = 2;
            }
        })
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("VAL: {}", unsafe {VAL});
}