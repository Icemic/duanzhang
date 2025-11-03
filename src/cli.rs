use clap::{Parser, Subcommand, ValueEnum};

/// 字体信息查询和子集化工具
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 查询字体信息
    Info { file: String },
    /// 创建字体子集
    Subset {
        file: String,

        /// 输出路径
        #[arg(short, long)]
        output: Option<String>,

        /// 字符集文件（包含所需字符的文本文件）
        #[arg(short, long, value_delimiter = ',')]
        charset: Vec<String>,

        /// 使用预设字符集
        #[arg(long, value_delimiter = ',')]
        presets: Vec<Preset>,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    /// 推荐字符集，包含所有中日韩统一表意文字、各种标点符号、日文假名、韩文谚文、注音符号、方块元素、Ascii字符
    Recommended,
    /// CJK 符号和标点
    Punctuation,
    /// 常用符号,
    Symbols,
    /// 注音符号
    Bopomofo,
    /// 全部中日韩统一表意文字（基本平面）
    CJKUnifiedIdeographs,
    /// 日文假名
    Kana,
    /// 谚文
    Hangul,
    /// Ascii 字符
    Ascii,
}

pub fn get_args() -> Args {
    Args::parse()
}
