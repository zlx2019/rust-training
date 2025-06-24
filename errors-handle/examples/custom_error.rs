use std::error::Error;
use std::fmt::{Display, Formatter};

///
/// 自定义Error
///

/// 自定义一个Error类型，使用枚举
#[derive(Debug)]
pub enum BusinessError {
    /// IO 错误
    IO(std::io::Error),
    /// 参数校验错误
    Validation {field: String, message: String},
    /// 未知错误
    Unknown(&'static str)
}

/// 实现 Error 的输出格式
impl Display for BusinessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BusinessError::Unknown(cause) => write!(f, "Unknown error: {}", cause),
            BusinessError::IO(err) => write!(f, "IO error: {}", err),
            BusinessError::Validation{field, message} => {
                write!(f, "Validation error: field: {}, message: {}", field, message)
            }
        }
    }
}

impl Error for BusinessError {
    /// 返回更底层的错误源（如果有）
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BusinessError::IO(err) => Some(err),
            _ => None
        }
    }
}

/// 实现 std::io:Error 到 BusinessError 错误的转换，使其可以通过 ? 向上传播
impl From<std::io::Error> for BusinessError {
    fn from(error: std::io::Error) -> Self {
        BusinessError::IO(error)
    }
}


fn main() -> Result<(), BusinessError> {
    let res: Result<(), BusinessError>;
    res = Err(BusinessError::IO(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found")));
    // return res;
    if let Err(ref e) = res {
        println!("{}", e);
    }
    Ok(())
}




