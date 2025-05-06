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


/// 编写一个 处理 Future 的宏, => Poll::Ready / Poll:Pending
/// 如果 Future 为 Ready 则返回 Future的值，否则返回 Pending
#[macro_export]
macro_rules! ready {
    ($fut: expr) => {
        match $fut {
            std::task::Poll::Ready(value) => value,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    }
}