
/// 编写我们的第一个宏：实现一个简化版的 `vec!`
#[macro_export]
macro_rules! vector {
    // vector![] => []
    // 当可变长参数为空时，则直接返回一个空的 Vec
    () => {Vec::new()};

    // vector![3, 4]; => [3,3,3,3]
    ($elem: expr; $size: expr) => {
        std::vec::from_elem($elem, $size)
    };

    // MacroMatcher 格式:  $ ( MacroMatch+ ) MacroRepSep? MacroRepOp
    // `$()` 表示括号内的模式可以重复多次
    // `$item: expr` 表示捕获片段的元变量名和捕获片段的类型为 一个表达式
    // ',' 表示重复匹配表达式之间的分隔符
    // '+' 表示可以重复1次或多次.
    // `$(,)?` 表示最后可以存在一个逗号也可以不存在.
    ( $( $item: expr ),+ $(,)?) => {
        {
            // 优化方式，展开入参的可变长元素
            <[_]>::into_vec(Box::new([$($item),+]))
        }
    }
}

fn main() {
    /// 使用我们自己编写的 vec 宏
    let vec1 =  vector![1,2,3];
    let vec2 = vector![3; 4];
    println!("{:?} {:?}", vec1, vec2);
}