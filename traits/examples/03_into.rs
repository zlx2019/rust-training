#![allow(unused_variables)]
#![allow(dead_code)]

/// `Into` 特征用于将自身转换为特定的类型。
/// 通常会用于 类型转换 和 泛型约束，注意，Into的转换是不能失败的，如果转换过程中可能出现错误请考虑使用`TryInto`。

#[derive(Debug)]
struct Vector2{
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Vector3{
    x: i32,
    y: i32,
    z: i32
}

/// 为 Vector2 实现 Into<Vector3> 特征，完成 Vector2 到 Vector3的转换. 
/// 注意，会转移 Vector2 的所有权
impl Into<Vector3> for Vector2{
    fn into(self) -> Vector3 {
        Vector3 { x: self.x, y: self.y, z: 0 }
    }
}

/// Vector3 to Vector2
impl Into<Vector2> for Vector3 {
    fn into(self) -> Vector2 {
        Vector2 { x: self.x, y: self.y }
    }
}

fn main(){
    let v2 = Vector2{x: 15, y: 32};
    let v3: Vector3 = v2.into();
    println!("{:?}", v3);

    let n_v2: Vector2 = v3.into();
    println!("{:?}", n_v2);
}