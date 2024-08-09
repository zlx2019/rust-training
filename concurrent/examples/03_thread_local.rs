/// Thread Local
/// 可以通过标准库的 `thread_local` 宏初始化线程局部变量

use std::{cell::RefCell, thread};

// 创建一个 u32 thread local，初始值为10
thread_local! {
    pub static FOO: RefCell<u32> = RefCell::new(10);
}


fn main(){

    // 创建子线程，将值修改为 99
    let task = thread::spawn(move || {
        FOO.with(|f| {
            *f.borrow_mut() = 99;
            println!("other thread f:{}", *f.borrow());
        });
    });
    task.join().unwrap();

    // 虽然子线程修改为了99，但是在主线程依然是10
    FOO.with(|f|{
        println!("main thread f:{}", *f.borrow())
    });
}