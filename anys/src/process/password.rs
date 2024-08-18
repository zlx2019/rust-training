#![allow(dead_code)]  // 忽略本文件的一些提示
use rand::seq::SliceRandom;

///
/// rand 命令相关处理
/// 


// 一些静态字符集
static UPER_CASE_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
static LOWER_CASE_CHARS: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
static NUMBER_CHARS: &[u8] = b"123456789";
static SPECIAL_CHARS: &[u8] = b"!@#$%^&*_";


/// 根据参数条件，生成随机字符密码
///  - `length`: 生成的密码长度;
///  - `upper`:  允许出现大写字符
///  - `number`: 允许出现数字
///  - `symbol`: 允许出现特殊字符
/// 
/// # Examples
/// 
/// ```
///     let passwd = gen_password(16, true, true, true, true);
///     assert_eq!(16, passwd.len());
/// 
/// ```
pub fn gen_password(length: u8, upper: bool, lower: bool, number: bool, symbol: bool) -> String{
    let mut password = Vec::new();
    let mut chars = Vec::new();
    let mut rng = rand::thread_rng();
    // 为了保证能包含指定的类型字符，每种字符需要单独生成一次
    if upper{
        chars.extend_from_slice(UPER_CASE_CHARS);
        password.push(*UPER_CASE_CHARS.choose(&mut rng).unwrap())
    }
    if lower {
        chars.extend_from_slice(LOWER_CASE_CHARS);
        password.push(*UPER_CASE_CHARS.choose(&mut rng).unwrap())
    }
    if number {
        chars.extend_from_slice(NUMBER_CHARS);
        password.push(*UPER_CASE_CHARS.choose(&mut rng).unwrap())
    }
    if symbol {
        chars.extend_from_slice(SPECIAL_CHARS);
        password.push(*UPER_CASE_CHARS.choose(&mut rng).unwrap())
    }
    // 补充剩余位
    let need = length - password.len() as u8;
    for _ in 0..need{
        // 通过 SliceRandom 随机获取字符集中随机字符
        let c = chars.choose(&mut rng).expect("charw won't be empty in this context");
        password.push(*c);
    }
    // 打乱字符顺序
    password.shuffle(&mut rng);
    return String::from_utf8(password).unwrap();
}