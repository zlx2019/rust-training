
/// 小练习：通过一个宏，快速构建出一个hashmap,
/// 如 maps!{"name": "ZhangSan", "age": 25}
#[macro_export]
macro_rules! maps {
    ($( $key: literal : $val: expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}

fn main() {
    let m = maps! {
        "name" : "ZhangSan",
        "age": "18"
    };
    println!("{:?}", m);
}