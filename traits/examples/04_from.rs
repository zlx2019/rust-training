#![allow(unused_variables)]
#![allow(dead_code)]

/// `From` 特征用于将某种特定的类型，转换为自身类型，恰好与 Into 相反。
/// 如果实现了 From, 则默认为目标类型实现了 Into 特征。
/// 如：
///     为类型A 实现 From<B> 特征，那么则默认为类型B 实现了 Into<A>特征。
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
/// 为 Vector3 实现来自于 Vector2 的转换，此转换会导致Vector2 失去所有权
impl From<Vector2> for Vector3{
    fn from(v2: Vector2) -> Self {
        Self { x: v2.x, y: v2.y, z: 0 }
    }
}

/// 通过引用来转换，不会失去所有权
impl From<&Vector2> for Vector3{
    fn from(v2: &Vector2) -> Self {
        Self { x: v2.x, y: v2.y, z: 0 }
    }
}


fn main(){
    let v2 =  Vector2{x: 10, y: 30};
    
    // 通过 Vector2 Into 为 Vector3
    let mut v3: Vector3 = v2.into();
    println!("{:?}", v3);
    

    // 通过 From 将Vector2 转换为 Vector3(通过借用)
    let v2 = Vector2{x: 100, y: 200};
    v3 = Vector3::from(&v2);
    println!("{:?}", v3);

    // 通过所有权转换，转换后v2将无法继续使用
    v3 = Vector3::from(v2);
    println!("{:?}", v3);

}