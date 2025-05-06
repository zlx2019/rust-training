/// 最简单的 声明式宏 使用


/// 通过`macro_rules!` 来声明宏
macro_rules! say_hello {
    // 无参数模式
    () => {
        println!("Hello Zeros!");
    };
    // 有参数模式
    ($name: expr) => {
        println!("Hello {}!", $name);
    }
}


/// 通过宏 实现 C# nameof
macro_rules! name_of {
    ($name: ident) => {
        stringify!($name)
    };
}

struct Person;

fn main() {
    say_hello!();
    say_hello!("Zeros");
    println!("person struct name is {}", name_of!(Person));
}