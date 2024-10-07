/// 枚举（enumeration）

/// note 不包含任何字段的枚举
enum Move{
    Up,
    Down,
    Left,
    Right
}

/// note 枚举项可以包含元组结构体
struct Point(f64, f64);
enum Move1{
    To(Point),
    Up(f64),
    Down(f64),
    Left(f64),
    Right(f64)
}

/// note 枚举项也可以包含普通结构体
enum Move2{
    Up{x: f64, y: f64, z: f64},
    Down{x: f64, y: f64, z: f64},
    Left{x: f64, y: f64, z: f64},
    Right{x: f64, y: f64, z: f64},
}

#[derive(Debug)]
enum Light{
    On = 1,
    Off = 2
}

fn main() {
    let _move1 = Move::Down;
    let _move2 = Move1::Up(10f64);
    let _move3 = Move2::Left {x: 1.00, y: 2.00, z: 3.00};
    let light = Light::On;
}