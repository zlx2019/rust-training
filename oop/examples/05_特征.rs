//! 特征（Trait）类似于其他语言中的接口（Interface）可以为类型定义一些共同行为。
//! 例如定义两种结构体，分别表示一些二维物体，如矩形、圆等物体，而这些物体都可以计算面积和周长
//! 那么我们就可以将计算面积和周长的行为，抽象成一种特征。

use std::any::Any;

/// 二维物体抽象特征
trait Plane{
    /// 计算物体的面积
    fn area(&self) -> f64;
    /// 计算物体的周长
    fn perimeter(&self) -> f64;
}

/// 矩形
struct Rectangle{
    height: f64,
    width: f64
}
/// 为 Rectangle 实现 Plane 特征
impl Plane for Rectangle{
    /// 计算矩形的面积
    fn area(&self) -> f64 {
        self.height * self.width
    }
    /// 计算矩形的周长
    fn perimeter(&self) -> f64 {
        (self.height + self.width) * 2f64
    }
}

/// 圆
struct Circle{
    radius: f64
}
/// 为 Circle 实现 Plane 特征
impl Plane for Circle{
    /// 计算圆的面积
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2.0)
    }
    /// 计算圆的周长
    fn perimeter(&self) -> f64 {
        std::f64::consts::PI * 2.0 * self.radius
    }
}


fn main() {
    let rec = Rectangle{height: 100f64, width: 50f64};
    println!("矩形的面积: {}, 周长: {}", rec.area(), rec.perimeter());
    let circle = Circle{radius: 50f64};
    println!("圆的半径为: {}, 面积: {:.2}, 周长: {:.2}",circle.radius, circle.area(), circle.perimeter())
}