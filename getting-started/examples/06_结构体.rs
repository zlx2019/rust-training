#![allow(dead_code)]

/// 结构体是多种类型组合而成的复合类型、如之前学到的i32、u64、bool、&str等类型组合在一起
/// 其实就等同于面向对象语言中的Class

/// 定义一个结构体
struct User{
    username: String,
    password: String,
    active: bool,
    sign_in_count: u64
}

/// 元组结构体，可以不用为字段定义名称
struct Point(i32, i32, u32);

fn main() {
    // 创建结构体实例
    let _ = User{
        username: "Zero".to_string(),
        password: "Root".to_string(),
        active: true,
        sign_in_count: 18,
    };



    let _ = Point(0,0,0);
}