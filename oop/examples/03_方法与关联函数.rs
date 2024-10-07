#![allow(dead_code)]
/// 方法: 属于结构体实例，通过实例可以调用
/// 关联函数: 属于结构体本身，可以直接通过结构体调用，类似于Java中的 static 方法.

/// 2维空间坐标
#[derive(Debug)]
struct Point{
    x: f64,
    y: f64
}

/// note 普通函数
/// 计算两点之间的距离
fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0)).sqrt()
}

impl Point{
    /// note 方法
    fn distance(&self, other: &Self) -> f64{
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }

    /// 关联函数
    fn dist(p1: &Self, p2: &Self) -> f64{
        ((p2.x - p1.x).powf(2.0) + (p2.y - p1.y).powf(2.0)).sqrt()
    }
}


fn main() {
    // 调用函数
    let p1 = Point{x: 10.00, y: 20.00};
    let p2 = Point{x: 25.64, y: 30.00};
    println!("{:?}", distance(&p1, &p2));

    // 调用方法
    let p3 = Point{..p1};
    let p4 = Point{..p2};
    println!("{:?}", p3.distance(&p4));
    // 以关联函数方式 也可以调用方法，但需要显示指定 self 的引用指向
    println!("{:?}", Point::distance(&p3, &p4));

    // 调用关联函数
    let p5 = Point{..p3};
    let p6 = Point{..p4};
    println!("{:?}", Point::dist(&p5, &p6));

}