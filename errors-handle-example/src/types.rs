
/// 自定义 Error & Result
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

