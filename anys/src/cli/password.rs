use clap::Parser;


/**
 * 生成随机字符序列 命令参数
 */
#[derive(Debug, Parser)]
pub struct GenPasswordOpts{
    /// 生成的密码长度
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    /// 允许出现大写字符
    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    /// 允许出现小写字符
    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    /// 允许出现数字
    #[arg(long, default_value_t = true)]
    pub number: bool,

    // --no-symbol 表示为不允许(true)
    /// 不允许出现特殊字符
    #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub no_symbol: bool,
}
