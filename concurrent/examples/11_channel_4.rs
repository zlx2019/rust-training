use std::sync::mpsc;

/// 一个消息只能传输一种类型数据，如果想要传输多种，可以通过多条通道，也可以通过枚举.
enum Fruit {
    Apple(u8),
    Other(String)
}

// 需要注意：Rust中的枚举占用内存大小，会以占用内存最大的成员为准而对齐
// 意味着哪怕你传输的是枚举中最小的成员，但是它的内存使用依然是和最大成员相同.
fn main(){
    let (tx, rx) = mpsc::channel();
    tx.send(Fruit::Apple(16)).unwrap();
    tx.send(Fruit::Apple(15)).unwrap();
    tx.send(Fruit::Other("Apple 14 pro max".to_string())).unwrap();

    for _ in 0..3 {
        match rx.recv().unwrap() {
            Fruit::Apple(v) => {
                println!("apple: {}", v);
            },
            Fruit::Other(s) => {
                println!("other {}", s);
            },
        }
    }
}