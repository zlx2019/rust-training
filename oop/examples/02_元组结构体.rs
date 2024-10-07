//! 元组结构体的字段可以没有名称,类似于元组，因此称之为元组结构体.

#[derive(Debug)]
struct Color(u8, u8, u8);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let red = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    println!("{:?}", red);
    println!("{:?}", origin);
}