

/// 定义模块
mod errors;
mod native_errors;


/// 导出模块资源
pub use errors::BusinessError;
pub use native_errors::NativeBusinessError;