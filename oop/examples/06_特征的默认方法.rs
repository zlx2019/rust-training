/// 特征和Java8以上的接口类似，可以拥有默认的方法实现

trait Behavior{
    // 定义一个函数，提供默认实现.
    fn sleep(&self){
        println!("default sleep");
    }

    fn walk(&self){
        println!("default walk");
    }

    fn run(&self);
}
struct Person;
impl Behavior for Person{
    /// 重写 run 方法
    fn run(&self){
        
        println!("persion run");
    }
}

fn show_behavior(param: impl Behavior){
    param.walk();
    param.sleep();
    param.run();
}


fn main() {
    show_behavior(Person);
}