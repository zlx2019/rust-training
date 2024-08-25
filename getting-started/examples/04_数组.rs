#![allow(dead_code)]
#![warn(unused_variables)]

/// 在日常开发中，数组(Array)是使用最为广泛的数据结构之一，在Rust中最常用的数组有两种：
///  - 第一种是速度很快，但是定长的 `Array`
///  - 第二种是可动态扩充，但是有性能损耗的 `Vector`
/// 我们称 `Array` 为数组，`Vector` 为动态数组，本节主要介绍 `Array`.
/// 数组三要素：
///  - 长度固定
///  - 类型相同
///  - 内存线性排列


/// 在Rust中，数组的定义如下：
fn create_array(){
    // 定义一个数组，类型和长度由元素列表推导
    let _arr = [1,2,3]; // 推导为 [i32;3] 类型
    assert_eq!(_arr.len(), 3);
    
    // 定义一个长度固定为5，u8 类型的数组
    let _arr2: [u8; 5] = [1,2,3,4,5];
    assert_eq!(_arr2.len(), 5);

    // 也可以在初始化时，将所有的元素初始化为相同值
    let _arr3 = [1; 5];
    assert_eq!(_arr3, [1,1,1,1,1]);
}

/// 因为数组中的数据是连续的，所以可以根据索引的方式来访问元素
fn access_elem(){
    let arr = [1,2,3,4];
    assert_eq!(arr[0], 1);
    assert_eq!(arr[1], 2);
    assert_eq!(arr[2], 3);
    assert_ne!(arr[3], 10);
}

/// 如果根据索引访问了不存在的元素，则会导致程序直接panic 崩溃
fn index_out_panic(){
    let _arr = [1,2,3];
    //let _elem = _arr[10];  // ^^^^^^^ index out of bounds: the length is 3 but the index is 10
}

/// 当数组元素为`非基础类型`
fn array_elem_complex(){
    // 如下情况
    // let hobby = [String::from("篮球") ;3]; // 编译错误

    // 这是因为 String 并非基本类型，它没有默认实现 Copy 特征，而[1;10]底层就是不停的Copy出来的元素
    // 所以只能一个个创建
    let _hobby = [String::from("唱"), String::from("跳"), "篮球".to_string()];
    // 啊！ 这样写简直辣眼睛! 下面是更优写法:
    let _hobby: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
}



fn main(){
    create_array();
    access_elem();
    index_out_panic();
}