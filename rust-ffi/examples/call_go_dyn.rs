// Rust 调用动态链接库函数（Go）
use std::{ffi::CString, os::raw::c_char};

// 声明外部的本地函数，并指定库名
#[link(name = "go_dyn", kind = "dylib")]
unsafe extern "C" {
    fn say_hello(input: *const c_char);
}

// 包装本地函数
fn wrapper_say_hello(input: &str){
    let rwa_str = CString::new(input).unwrap();
    unsafe {
        say_hello(rwa_str.as_ptr());
    }
}

fn main(){
    wrapper_say_hello("Hello rust + go!");
}