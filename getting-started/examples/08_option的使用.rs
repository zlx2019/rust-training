#![allow(dead_code)]
#![allow(unused_variables)]

/// Option 的方法介绍
/// ```
///pub enum Option<T> {
///   None,
///   Some(T),
///}
/// ```
///     - is_some(): 是否为 Some(T)。
///     - is_none(): 是否为 None。
///     - is_some_and(): 是否为 Some(T) 并且谓词函数也返回 true。
///     - is_none_or(): 是否为 None，或者谓词函数返回 true。
///     - 
///     - 
/// 
/// 
/// 


/// is_some(&self) -> bool
/// 
/// 判断 Option 是否为 Some。
fn is_some_example(){
    let x = Some(2);
    assert_eq!(x.is_some(), true);
    
    let x: Option<i32> = None;
    assert_eq!(x.is_some(), false);
}

/// is_none(&self) -> bool
fn is_none_example(){
    let x: Option<i32> = Some(2);
    assert_eq!(x.is_none(), false);

    let x: Option<i32> = None;
    assert_eq!(x.is_none(), true);
}

/// is_some_and(self, FnOnce -> bool) -> bool
/// 
/// 如果是 Some，并且谓词函数闭包返回true，则返回true。
fn is_some_and_example(){
    let x = Some(23);
    assert_eq!(x.is_some_and(|v| v == 23), true);
    let x = Some(32);
    assert_eq!(x.is_some_and(|v| v == 30), false);
}


/// is_none_or(self, FnOnce -> bool) -> bool
/// 
/// 如果是 None 或者谓词闭包函数返回 true，则返回true。
fn is_none_or_example(){
    // 谓词函数匹配，返回 true。
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none_or(|v| v == 2), true);

    // 不是 None，并且谓词不匹配，返回 false
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none_or(|v| v == 3), false);

    // None 返回true
    let x: Option<u32> = None;
    assert_eq!(x.is_none_or(|v| v == 2), true);
}

/// as_ref(&self) -> Option<&T>
/// 
/// 将 Option 内部值转换成引用类型。
fn as_ref_example(){
    let text: Option<String> = Some(String::from("Hello, world!"));
    let length = text.as_ref().map(|s|s.len()).unwrap_or_default();
    assert_eq!(length, 13);
}


/// as_mut() -> Option<&mut T>
/// 
/// 将 Option 内部的值转换成可变引用类型。
fn as_mut_example(){
    let mut text: Option<String> = Some("Hello, ".into());
    let v = text.as_mut().map(|s|{
        s.push_str("world!");
        s.to_string()
    });
    assert_eq!(v, Some(String::from("Hello, world!")));
}

/// as_slice(&self) -> &[T]
/// 
/// 将 Option 转换为 &[T] 切片，切片元素为原始值的只读引用。
fn as_slice_example(){
    let list = Some(123);
    let d = list.as_slice();
    assert_eq!(list.as_slice(), vec![123]);

    let list: Option<i32> = None;
    assert_eq!(list.as_slice(), vec![])
}

/// as_mut_slice(&self) -> &mut [T]
/// 
/// 将 Option 转换为 切片类型，并且内部元素为可变引用。
fn as_mut_slice_example(){
    let mut list = Some(123);
    let s = list.as_mut_slice();
    s[0] = 1024;
    assert_eq!(s, vec![1024]);
}

/// expect(self, &str) -> T
/// 
/// 返回 Some的值，如果为None则直接panic，输出自定义的消息。
fn expect_example(){
    let x = Some("value");
    assert_eq!(x.expect("is none."), "value");

    let x: Option<&str> = None;
    // x.expect("x should not be empty"); // panics with `is none.`
}

/// unwrap(self) -> T
/// 
/// 返回 Some 的值，如果是None，则直接panic。
fn unwrap_example(){
    let x = Some(2);
    assert_eq!(x.unwrap(), 2);
}

/// unwrap_or(self, T) -> T
/// 
/// 返回 Some的值，如果是None，则返回传入的默认值
fn unwrap_or_example(){
    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike")
}

fn unwrap_or_else_example(){
    
}

fn main(){
    is_some_example();
    is_none_example();
    is_some_and_example();
    is_none_or_example();

    as_ref_example();
    as_mut_example();
    as_slice_example();
    as_mut_slice_example();

    expect_example();

    unwrap_example();
    unwrap_or_example();
}