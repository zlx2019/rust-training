/// 最简单的 声明宏（declarative macros）


/// 通过`macro_rules!` 来声明宏
macro_rules! say_hello {
    // 无参数模式
    () => {
        println!("Hello Zeros!");
    };
    // 带一个参数模式，参数类型为一个一句表达式
    ($name: expr) => {
        println!("Hello {}!", $name);
    };
    // 带两个参数模式
    // 参数1：表达式
    // 参数2：字面值
    ($name: expr, $content: literal) => {
        println!("{}: {}", $name, $content);
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
    say_hello!("Zero", "你好");


    println!("person struct name is {}", name_of!(Person));
}