#![allow(unused_variables)]
#![allow(dead_code)]

use std::{ffi::{CStr, CString}, os::raw::c_char};

//
// 通过 Rust ffi 调用 C 语言静态函数
// 


// 声明外部的本地函数
unsafe extern "C" {
    /// 字符串转大写
    fn str_to_upper(arg: *const c_char) -> *mut c_char;
    /// 释放字符串内存(C)
    fn free_c_str(arg: *mut c_char);
}

// 包装本地函数
fn wrapper_str_to_upper(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 将 Rust 字符串转换为 C 字符串
    let rwa_str = CString::new(input)?;

    // 调用本地C函数
    let ret_raw_ptr = unsafe {
        str_to_upper(rwa_str.as_ptr())
    };
    if ret_raw_ptr.is_null() {
        // C 返回 NULL
        return Err("C func return null pointer".into())
    }

    // 将C原始字符串转换为 Rust 字符串
    let ret_val = unsafe {
        let c_str = CStr::from_ptr(ret_raw_ptr);
        let rs_string = c_str.to_string_lossy().into_owned();
        // 释放 C 分配的字符串内容
        free_c_str(ret_raw_ptr);
        rs_string
    };
    Ok(ret_val)
}

fn main(){
        println!("Rust call C lib");
    let lines = vec![
        "java",
        "rust",
        "golang"
    ];
    for ele in lines {
        match wrapper_str_to_upper(ele) {
            Ok(ret) => {
                println!("input: {} ==> output: {}", ele, ret);
            },
            Err(e) => {
                eprintln!("error process {} : {}", ele, e);
            }
        }
    }
}