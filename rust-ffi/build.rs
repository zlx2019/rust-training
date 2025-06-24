const C_SOURCE_FILE: &'static str = "src/hello.c";

/// 构建脚本
fn main(){
    // 当指定文件发生变更后，就重新运行当前的构建脚本
    println!("cargo:rerun-if-changed={}", C_SOURCE_FILE);
    // 将C源文件编译为静态库
    cc::Build::new().file(C_SOURCE_FILE).compile("hello");
}