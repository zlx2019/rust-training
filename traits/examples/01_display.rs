/// `std::fmt::Display` 特征主要用于类型格式化输出，提供类型输出时显示的内容.
/// 用于使用 {} 占位符格式化输出.
/// 实现此特征后，将自动实现 `ToString` trait, 允许使用 `.to_string()` 方法，所以优先实现 Display 而不是 ToString
/// 
/// 常用的格式化 trait：
/// std::fmt::Display - 用于 {}
/// std::fmt::Debug - 用于 {:?}
/// std::fmt::Octal - 用于 {:o}
/// std::fmt::Binary - 用于 {:b}
/// std::fmt::LowerHex - 用于 {:x}
/// std::fmt::UpperHex - 用于 {:X}


struct Point{
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Porint(x: {}, y: {})", self.x, self.y)
    }
}

impl std::fmt::Binary for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Porint(x: {:b}, y: {:b})", self.x, self.y)
    }
}

fn main(){
    let p =  Point{x: 10, y: 30};
    println!("{}", p);
    let s = p.to_string();
    println!("{s}");

    assert_eq!(format!("{}", p), "Porint(x: 10, y: 30)");
    assert_eq!(format!("{:b}", p), "Porint(x: 1010, y: 11110)");
}