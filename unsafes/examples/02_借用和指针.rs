/// 在Rust中，同时存在借用和裸指针
///     - 借用: 
///         - 不可变借用 &T
///         - 可变借用   &mut T
///     - 裸指针:
///         - 不可变指针  *const T
///         - 可变指针    *mut T
/// 

fn main(){
    let mut x: i32  = 10;  // 定义变量

    // 获取不可变借用
    let r1: &i32 = &x;
    println!("ref: {}", r1);
    // 获取可变借用
    let r2: &mut i32 = &mut x;
    println!("mut ref: {}", r2);

    // 可以将借用强转为指针
    // 获取可变指针
    let p1: *const i32 = &x;
    // 获取不可变指针
    let p2: *mut i32 = &mut x;
    unsafe{
        println!("ptr1: {}", *p1);
        println!("ptr2: {}", *p2);
    }
}