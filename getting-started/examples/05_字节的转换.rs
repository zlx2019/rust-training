#![allow(dead_code)]
#![warn(unused_variables)]

/// 在网络编程中，字节流的处理是不可避免的，它非常重要!!!
/// 在Rust中，`u8` 类型就是所谓的Byte,字节类型，所以字节流也是 [u8] 数组


/// 字符串 转 字节序列
fn string_to_bytes(){
    // String &str -> &[u8]
    let s1 = "Hello,World".to_string();
    let bytes = s1.as_bytes(); 
    println!("{:?}", bytes); // [72, 101, 108, 108, 111, 44, 87, 111, 114, 108, 100]
    let s2 = "Hello,World";
    let bytes2 = s2.as_bytes();
    assert_eq!(bytes, bytes2);

    // String &str -> Vec<u8>
    let bytes3 = "Hello".as_bytes().to_vec();
    let bytes4 = "Hello".to_string().into_bytes();
    assert_eq!(bytes3, bytes4);
}

/// 字节序列 转 字符串
fn bytes_to_string(){

    // &[u8] -> &str
    let bytes: &[u8] = &[72, 101, 108, 108, 111, 44, 87, 111, 114, 108, 100];
    if let Ok(s) = std::str::from_utf8(bytes) {
        println!("{}", s); // Hello,World
    }

    // Vec[u8] -> String
    let bytes2: Vec<u8> = vec![72, 101, 108, 108, 111, 44, 87, 111, 114, 108, 100];
    if let Ok(s) = String::from_utf8(bytes2){
        println!("{}", s);
    }
}

fn main(){
    string_to_bytes();
    bytes_to_string();
}