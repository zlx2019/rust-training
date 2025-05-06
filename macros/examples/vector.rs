use macros::vector;

fn main() {
    /// 使用我们自己编写的 vec 宏
    let vec1 =  vector![1,2,3];
    let vec2 = vector![3; 4];
    println!("{:?} {:?}", vec1, vec2);
}