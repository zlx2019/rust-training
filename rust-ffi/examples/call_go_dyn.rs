#![allow(unused_variables)]
#![allow(dead_code)]

use std::{ffi::{CStr, CString}, os::raw::c_char};


// 
// 通过 Rust ffi 调用动态链接库函数（Go）
//

// 声明外部动态库的函数
#[link(name = "go_dyn", kind = "dylib")]
unsafe extern "C" {
    fn print_hello(input: *const c_char);
    fn transform(input: *const c_char) -> *mut c_char;
    fn free_str(input: *mut c_char);
}

// print_hello 包装方法
fn wrapper_print_hello(input: &str){
    let rwa_str = CString::new(input).unwrap();
    unsafe {
        print_hello(rwa_str.as_ptr());
    }
}

// transform 包装方法
fn wrapper_transform(input: &str) -> Result<String, Box<dyn std::error::Error>>{
    let raw_str = CString::new(input)?;
    unsafe {
        let ret_ptr =  transform(raw_str.as_ptr());
        if ret_ptr.is_null() {
            return Err("Go retuen null addr ".into());
        }
        let output = CStr::from_ptr(ret_ptr).to_string_lossy().to_string();
        free_str(ret_ptr); // 释放 Go 字符串内存
        Ok(output)
    }
}



fn main(){
    wrapper_print_hello("Hello rust + go!");
    let ret = wrapper_transform("rust go go go!").unwrap();
    println!("ret: {ret}");
}