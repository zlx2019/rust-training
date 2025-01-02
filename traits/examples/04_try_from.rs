#![allow(unused_variables)]
#![allow(dead_code)]

use std::io::Error;


/// 
/// `TryFrom` 特征用于转换可能发生失败的类型转换，返回Result、
/// 

pub struct Email(String);

impl TryFrom<&str> for Email{
    type Error = std::io::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains('@') {
            Ok(Email(value.to_string()))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "不包含 `@` 符号."))
        }
    }
}

fn main(){
    assert!(Email::try_from("wdamnjkdnwasjzwsa").is_err());
    assert!(Email::try_from("xnjksad@as.com").is_ok());

    let r: Result<Email, Error> = "sadwa".try_into();
    assert!(r.is_err());
    let r: Result<Email, Error> = "dwkja@gmail.com".try_into();
    assert!(r.is_ok());
}