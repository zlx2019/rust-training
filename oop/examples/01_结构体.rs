//! Rust 同样支持面向对象特性，其中结构体(Struct)担任着非常重要的角色，类似于Class。
//!   结构体与元组(Tuple)有些相似，可以由多种不同的数据类型组合而成，与元组不同的是结构体可以为内部每个字段起一个名称
//! 因此结构体更加灵活强大，你无需依赖字段的顺序来访问和操作它们。

/// 定义结构体
#[derive(Debug)]
struct User{
    username: String,
    password: String,
    nike_name: &'static str,
    age: u8,
    active: bool
}

/// 创建 `User` 结构体的实例
fn create_user(){
    // 1. 初始化时，每个字段都需要进行初始化.
    // 2. 初始化时的字段顺序没有要求
    let mut user = User{
        username: "zero9501".to_string(),
        password: "root@ROOT".to_string(),
        nike_name: "zero",
        active: false,
        age: 20,
    };
    // 通过`.`即可访问字段值
    println!("username: {}, nick_name: {}", user.username, user.nike_name);
    // 也可以修改字段
    user.age = 25;
}

/// 简化创建 `User` 实例
fn build_user(username: String, password: String, nick_name: &'static str) -> User{
    User{
        username,
        password,
        nike_name: nick_name,
        age: 18,
        active: true
    }
}

/// 根据已有的实例，创建出一个新的实例
fn fill_update(){
    let user1 = build_user("user".to_string(), "root".to_string(), "user1");
    let user2 = User{
        username: "user2".to_string(),
        ..user1
    };

    println!("{:?}", user1.username);
    // println!("{:?}", user1.password);
    println!("{:?}", user2);
}

fn main() {
    create_user();
    fill_update();
}