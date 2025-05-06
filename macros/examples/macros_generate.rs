/// 通过宏来创建 方法或结构体等等


/// 定义方法
macro_rules! define_fn {
    // name: 方法名 --> 标识符
    // rt:   返回值类型 --> 类型
    // body: 方法体 --> 代码块
    ($name: ident, $rt: tt, $body: block) => {
        fn $name () -> $rt {
            $body
        }
    };
}

/// 定义结构体
macro_rules! define_struct {
    // 结构体名称
    // 结构体属性列表 field : type
    (
        $name: ident,
        $( $field_name:ident : $field_type:ty ),*
    ) => {
        #[derive(Debug)]
        struct $name {
            // 遍历属性类型列表
            $( $field_name: $field_type),*
        }
        impl $name {
            fn new ($($field_name: $field_type),*) -> Self {
                Self{
                    $($field_name),*
                }
            }
        }
    };
}


fn main() {
    // 通过宏定义局部方法
    define_fn!(sum, i32, {
        let x = 10;
        let y = 10;
        x * y
    });
    let res=  sum();
    println!("x * y = {}", res);

    // 通过宏定义结构体
    define_struct!(User, name: String, age: u32, address: String);
    let user = User{name: String::from("Zero"), age: 18, address: "深圳".to_string()};
    println!("{:?}", user)
}