#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::{Error, ErrorKind};


/// Result 各种方法详解
/// 
/// ```
/// pub enum Result<T, E> {
///     Ok(T),
///     Err(E),
/// }
/// ```
/// 
///     - is_ok(): Result 是否为 Ok(T)
///     - is_err(): Result 是否为 Err(E)
///     - is_ok_and(): 当Result为 Ok(T) 并且谓词函数返回 true 时返回 true。
///     - is_err_and(): 当Result为 Err(E) 并且谓词函数返回 true 时返回 true。
///     - ok(): 将 Result<T, E> 转换为 Option<T>
///     - err(): 将 Result<T, E> 转换为 Option<E>
/// 
///     - as_ref(): 将 Result<T, E> 转换为 Result<&T, &E>
///     - as_mut(): 将 Result<T, E> 转换为 Result<&mut T, &mut E>
/// 
///     - map(): 将Ok的值 映射处理为新的值。
///     - map_err(): 将Err的值，映射处理为新的值。
/// 
///     - unwrap(): 获取 Ok 的值, 如果是 Err 则会panic。
///     - unwrap_err(): 获取 Err 的值，如果是 Ok 则会panic。
///     - unwrap_or_default(): 获取 Ok 的值，如果是 Err 则返回T类型的零值。
///     - unwrap_or(): 获取Ok的值，如果是 Err 则返回指定的默认值。
///     - unwrap_or_else(): 获取Ok的值，如果是 Err 则使用闭包返回的值作为返回值。
///     - unwrap_unchecked(): 获取 Ok 的值，不进行是否为 Err 的检查。
///     - unwrap_unchecked_err(): 获取 Err 的值，不进行是否为 Ok 的检查。
/// 
///     - expect(): 获取 Ok 的值，如果是 Err 则 panic 并且使用指定的消息作为错误消息。
///     - expect_err(): 同上，获取的是 Err 的值。
/// 
///     - into_ok(): 获取 Ok 的值，但保证此方法绝对不会为 Err。
///     - into_err(): 与 into_ok() 相反。
/// 
///     - and(): 如果两个Result都为Ok，返回第二个Result。
///     - and_then(): 
/// 


/// is_ok(&self) -> bool: 是否为 Ok(T)
/// is_err(&self) -> bool: 是否为 Err(E)
/// 不会移动所有权
fn is_ok_example(){
    let x: Result<i32, &str> = Ok(1);
    assert_eq!(x.is_ok(), true);
    assert_eq!(x.is_err(), false);

    let y: Result<i32, &str> = Err("Some error message");
    assert_eq!(y.is_ok(), false);
    assert_eq!(y.is_err(), true);
}

/// is_ok_and(self, FnOnce -> bool) -> bool
/// 当 Result 为 Ok(T)时并且谓词函数返回为 true 时才返回 true。
/// 
/// is_err_and(self, FnOnce -> bool) -> bool
/// 当 Result 为Err(E)时并且谓词函数返回为 true 时才返回 true。
fn is_ok_and_example(){
    let x: Result<u32, &str> = Ok(1024);
    assert_eq!(x.is_ok_and(|v| v == 1024), true);
    assert_eq!(x.is_ok_and(|v| v == 1025), false);

    let y: Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "file not found"));
    assert_eq!(y.is_err_and(|e| e.kind() == ErrorKind::NotFound), true);
    let z : Result<u32, Error> = Err(Error::new(ErrorKind::NotFound, "file not found"));
    assert_eq!(z.is_err_and(|e| e.kind() == ErrorKind::PermissionDenied), false);
}


/// ok(self) -> Option<T>
/// 
/// 将 Result<T,E> 转换为 Option<T>，直接丢弃错误。
///     - Result 为 Ok(T) 时，则转换为 Some(T)
///     - Result 为 Err(E) 时，则转换为 None
fn ok_example(){
    let x :Result<u32, &str> = Ok(2);
    assert_eq!(x.ok(), Some(2));
    assert_eq!(x.err(), None);
}

/// err(self) -> Option<E> 
/// 
/// 将 Result<T, E> 转换为 Option<E>，直接丢弃值。
///     - Result 为 Ok(T) 时，则转换为 None
///     - Result 为 Err(E) 时，则转换为 Some(E)
fn err_example(){
    let x: Result<u32, &str> = Err("Nothing here");
    assert_eq!(x.err(), Some("Nothing here"));
    assert_eq!(x.ok(), None);
}

/// as_ref() -> Result<&T, &E>
/// 
/// 将 Result<T, E> 转换为 Result<&T, &E>
/// 将Result内部的值，转换为只读引用。 生成一个新的Result，其中包含原始数据的引用,不会移动原始数据的所有权。
fn as_ref_example(){
    let x: Result<u32, String> = Ok(1024);
    assert_eq!(x.as_ref(), Ok(&1024));

    let y: Result<u32, &str> = Err("Error");
    assert_eq!(y.as_ref(), Err(&"Error"));
}

/// as_mut() -> Result<&mut T, &mut E>
/// 
/// 与`as_ref`方法类似，不同的是该方法是转换成可变引用。
fn as_mut_example(){
    let mut result: Result<String, &str> = Ok(String::from("hello"));
    // 默认Result中的值是不可变的，通过 as_mut 则修改成可变的。
    if let Ok(val) = result.as_mut() {
        val.push_str(", world!");
    }
    assert_eq!(result.unwrap(), "hello, world!".to_string());
}

/// map() -> Result<U, E>
/// 
/// 当 Result 为Ok<T>时，将 T 映射为另一种类型，返回 Result<U, E>
/// Err 值保持不变
fn map_example(){
    let kb: Result<i32, &str> = Ok(1024);
    let mb = kb.map(|v| (v << 10).to_string());
    assert_eq!(mb.as_ref().unwrap(), &"1048576".to_string());
    let gb = mb.map(|v| v.parse::<i32>().unwrap() << 10);
    assert_eq!(gb.unwrap(), 1073741824);
}

fn map_err_example(){
    let x: Result<i32, Error> = Err(ErrorKind::NotFound.into());
    let err = x.map_err(|e|{
        Error::new(ErrorKind::PermissionDenied, "权限不足")
    });
    assert_eq!(err.as_ref().unwrap_err().to_string(), "权限不足".to_string());
    assert_eq!(err.unwrap_err().kind(), ErrorKind::PermissionDenied);
}

/// map_or(def_val, FnOnce) -> U
/// 
/// 将Ok(T)的值映射为新的类型并且作为返回值，如果是Err时，则直接返回提供的默认值。
fn map_or_example(){
    let x: Result<i32, &str> = Ok(1024);
    let val = x.map_or(0, |v| v * 2);
    assert_eq!(val, 2048);

    let y: Result<i32, &str> = Err("fail");
    // 由于是Err,所以会返回默认值 `-1`
    let val = y.map_or(-1, |v| v * 2);
    assert_eq!(val, -1);
}

/// map_or_else(FnOnce, FnOnce) -> U
/// 
/// 该方法是 `map_or`的扩展版，通过闭包来获取默认值
fn map_or_else_example(){
    let x: Result<i32, &str> = Ok(1024);
    let val = x.map_or_else(|_|{-1}, |v|{v * 10});
    assert_eq!(val, 10240);

    let x: Result<i32, &str> = Err("fail");
    let val = x.map_or_else(|e|{-1}, |v|{v * 10});
    assert_eq!(val, -1);
}

/// unwrap(self) -> T
/// 
/// 返回 Ok(T) 的值，此方法会转移所有权。
/// 如果 Result 是 Err的话，则会直接panic。
fn unwrap_example(){
    let x: Result<u32, &str> = Ok(2);
    assert_eq!(x.unwrap(), 2);
}

/// unwrap_or_default(self) -> T
/// 
/// 返回Ok(T)的值，如果是Err则返回类型的零值。
fn unwrap_or_default_example(){
    let x: Result<i32, &str> = Ok(1024);
    assert_eq!(x.unwrap_or_default(), 1024);

    let x: Result<i32, &str> = Err("fail");
    assert_eq!(x.unwrap_or_default(), 0);
}

/// unwrap_or(self, default) -> T
/// 
/// 获取Ok(T)的值，如果为 Err 则返回提供的默认值。
fn unwrap_or_example(){
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or(0), 9);

    let x: Result<u32, &str> = Err("fail");
    assert_eq!(x.unwrap_or(0), 0);
}

/// unwrap_or_else(self, FnOnce) -> T
/// 
/// 获取Ok的值，如果为Err 则返回闭包提供的默认值。
fn unwrap_or_else_example(){
    let x: Result<u32, &str> = Ok(9);
    assert_eq!(x.unwrap_or_else(|_| 0), 9);

    let x: Result<u32, &str> = Err("fail");
    assert_eq!(x.unwrap_or_else(|_| 0), 0);
}

/// unwrap_unchecked(self) -> T
/// 这是 unsafe 版本的 unwrap()方法，该方法会直接获取Ok的值，而不会检查是否为Err
/// 
/// unwrap_err_unchecked(self) -> E
/// 这是 unsafe 版本的 unwrap_err()方法，该方法会直接获取Err的错误，而不会检查是否为Ok
fn unwrap_unchecked_example(){
    let x: Result<u32, &str> = Ok(9);
    unsafe {
        assert_eq!(x.unwrap_unchecked(), 9);
    }
    let x: Result<u32, &str> = Err("emergency failure");
    assert_eq!(unsafe {x.unwrap_err_unchecked()}, "emergency failure");
}

/// expect(self, msg) -> T
/// 返回 Ok(T) 的值，如果是 Err 则会 panic，将指定的信息作为错误信息。
/// 
/// expect(self, msg) -> E
/// 返回 Err 的值，如果是 Ok 则会 panic，将指定的信息作为错误信息。
fn expect_example(){
    let x: Result<i32, &str> = Ok(1024);
    assert_eq!(x.expect("is err"), 1024);

    let x: Result<i32, Error> = Err(ErrorKind::NotFound.into());
    assert_eq!(x.expect_err("is ok").kind(), ErrorKind::NotFound);
}

/// and(self, Result<U, E>) -> Result<U, E> 
/// 
/// 判断两个 Result 是否都为 Ok, 只有两个都是Ok时，会返回入参的Result。
///     - 如果有一个为Err，那么返回的必定是 Err。
fn and_example(){    
    // 当 x 和 y 都是Ok时，返回 y 的 Ok(T)值
    let x: Result<i32, &str> = Ok(2);
    let y: Result<&str, &str> = Ok("foo");
    assert_eq!(x.and(y), Ok("foo"));

    // 当 x 是Ok, y 是Err时，返回 y。
    let x: Result<i32, &str> = Ok(2);
    let y: Result<&str, &str> = Err("late error");
    assert_eq!(x.and(y), Err("late error"));

    // 当 x 是Err时
    let x: Result<i32, &str> = Err("late error");
    let y: Result<i32, &str> = Ok(2);
    assert_eq!(x.and(y), Err("late error"));

    // 当 x 和 y 都是 Err时，返回第一个Err。
    let x: Result<i32, &str> = Err("x error");
    let y: Result<i32, &str> = Err("y error");
    assert_eq!(x.and(y), Err("x error"));
}

/// and_then(self, FnOnce) -> Result<U, E>
/// 
/// 当 Result 为Ok时，调用传入的闭包函数。
fn and_then_example(){
    let x: Result<i32, &str> = Ok(1024);
    let val = x.and_then(|v| Ok(v << 10))
        .and_then(|v| Ok(v.to_string()))
        .unwrap_or("default_val".to_string());
    assert_eq!(val, String::from("1048576"));
}


fn or_example(){
    
}

/// inspect(self, FnOnce) -> Self
/// 
/// 当Result 为 Ok 时，执行传入的闭包函数。一般用于记录日志。
fn inspect_example(){
    let x: Result<i32, &str> = Ok(2);
    let v = x.inspect(|v|  println!("Ok of value: {}", v))
        .unwrap_or(0);
    assert_eq!(v, 2);
}

/// inspect_err(self, FnOnce) -> Self
/// 
/// 当 Result 为 Err时，执行传入的闭包函数，一般用于记录Error日志。
fn inspect_err_example(){
    let x: Result<i32, &str> = Err("custom error");
    let e = x.inspect_err(|e| println!("error: {}", *e)).unwrap_err();
    assert_eq!(e, "custom error");
}

/// iter(&self) -> Iter<_, T>
/// 
/// 将 Result 转换为 迭代器，Ok的值作为唯一的迭代元素。
fn iter_example(){
    let x: Result<u32, &str> = Ok(1024);
    assert_eq!(x.iter().next(),Some(&1024));

    let x: Result<u32, &str> = Err("nothing!");
    assert_eq!(x.iter().next(), None);


    let s: Result<Vec<u32>, &str> = Ok(vec![1,2,3,4,5]);
    for item in s.iter().flat_map(|v| v.iter()) {
        print!("{}, ", item);
    }
    println!();
}


/// iter_mut(&self) -> Iter<_, T>
/// 
/// 将 Result 转换为 可变迭代器，Ok值作为唯一的迭代元素。
fn iter_mut_example(){
    let mut x: Result<i32, &str> = Ok(1024);
    let mb = x.iter_mut()
        .next()
        .map(|v| *v << 10)
        .unwrap_or(0);
    assert_eq!(mb, 1048576);
}

fn main(){
    is_ok_example();
    is_ok_and_example();

    // Result -> Option 转换
    ok_example();
    err_example();
    
    // 将 Result 内部的值转换成引用/可变引用
    as_ref_example();
    as_mut_example();

    // map 映射处理
    map_example();
    map_err_example();
    map_or_example();
    map_or_else_example();

    // unwrap 系列: 直接获取 Ok 或 Err，若失败直接panic。
    unwrap_example();
    unwrap_or_default_example();
    unwrap_or_example();
    unwrap_unchecked_example();

    // expect 自定义错误消息系列
    expect_example();

    // 判断是否都为 Ok
    and_example();
    // 链式处理 Result 的Ok 的值。
    and_then_example();

    // inspect 检查系列函数，一般用于当Result 为 Ok 或 Err 时记录日志。
    inspect_example();
    inspect_err_example();

    // 转换为 迭代器
    iter_example();
    iter_mut_example();
}