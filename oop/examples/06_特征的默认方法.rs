//! 特征和Java8以上的接口类似，可以拥有默认的方法实现

trait Behavior{
    // 定义一个函数，提供默认实现.
    fn sleep(&self){
        println!("default sleep");
    }
}
struct Person;
impl Behavior for Person{}

fn main() {
    Person.sleep();
}