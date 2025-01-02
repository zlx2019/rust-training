/// `std::fmt::Debug` 特征主要用于调试上下文的输出。
/// 使用 {:?} 占位符, 使用 {:#} 能够美化输出

struct Point{
    x: i32,
    y: i32,
}
impl std::fmt::Debug for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()
    }
}


// Debug 特征可以通过 derive(派生宏) 直接实现
#[derive(Debug)]
struct Rec{
    height: u32,
    width: u32, 
}


fn main(){
    let p = Point{x: 10, y: 30};
    println!("{:?}", p);
    println!("{:#?}", p);

    let rec =  Rec{height: 100, width: 300};
    println!("{:?}", rec);
    println!("{:#?}", rec);
    
    // 命名参数格式化
    println!("{height} -- {width}", height=rec.height, width=rec.width);
    // 位置参数格式化
    println!("{0} -- {1} -- {2:?}", rec.height, rec.width, rec);
}