use std::env;

const C_SOURCE_FILE: &'static str = "src/hello.c";

/// 构建脚本
fn main(){
    // 将 C 源文件编译为静态库
    cc::Build::new().file(C_SOURCE_FILE).compile("hello");

    // 加载 Go 动态链接库
    // 获取当前目录
    let current_dir = env::current_dir().unwrap();
    // 指定动态库查找路径
    println!("cargo:rustc-link-search=native={}", current_dir.display());
    // 指定动态库名称（不包含前缀后缀）
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=golib");

    // 当指定文件发生变更后，就重新运行当前的构建脚本
    println!("cargo:rerun-if-changed={}", C_SOURCE_FILE);
    // 如果 Go 源文件发生变化，重新构建
    println!("cargo:rerun-if-changed=src/hello.go");
}