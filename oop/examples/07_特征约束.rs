//! Trait Bound 特征约束用于指定某个类型必须实现哪些特征
//! 特征约束通常用于函数参数、返回值或类型定义中.

use std::fmt::Debug;

/// 定义特征
trait Behavior{
    fn sleep(&self){
        println!("default sleep");
    }
}

/// 定义 结构体，实现 Behavior 特征.
struct Person;
impl Behavior for Person{
    fn sleep(&self) {
        println!("sleep for person");
    }
}

/// 该方法参数要求 必须实现了 Behavior 特征
fn sleep(obj: impl Behavior){
    obj.sleep();
}

/// 除了通过特征约束，还可以通过泛型来实现
fn generic_sleep<T: Behavior>(obj: T){
    obj.sleep();
}

/// 当然，也可以同时指定多个特征约束
fn sleep2(obj: impl Behavior + Debug){
    println!("{:?}", obj);
}

fn main() {
    sleep(Person);
    generic_sleep(Person);
}