
/// 过程宏（procedural macros）
///  - 派生宏 Derived macros
fn main() {

}


/// derive 派生宏
#[derive(Debug, Default)]
struct User;

/// 宏展开后的样子
impl core::fmt::Debug for User {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Formatter::write_str(f, "User")
    }
}
impl core::default::Default for User {
    #[inline]
    fn default() -> Self {
        User{}
    }
}