//! 通过 `std::io::copy` 可以方便的实现io的拷贝

fn main() {
    // 将标准输入中的数据，拷贝到标准输出中
    std::io::copy(&mut std::io::stdin(), &mut std::io::stdout()).unwrap();
}