//! # crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/**
 * 三个斜杠的注释可以生成html，可以用md语法。用cargo doc --open命令在target目录生成文档
 * 这里使用了Examples标题，另外常用的标题还有Panics，Errors和Safety
 * Panics把可能导致panic的用法列出
 * Errors把返回Result中可能出错的情况列出
 * Safety把unsafe的情况列出
 * 执行cargo test时，会执行文档注释中的代码
 * //!表示对整个文件的文档注释，他们必须出现在最前面，不然会报错
 * 发布crate时先登录cargo login LizXTHqWzoPeayGSu9kVVSGx8qQOW3Ny，输入命令后会存在~/.cargo/credentials中
 */

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
        SecondaryColor::Green
    }
}

// 对外从utils和kinds提供出去的接口不方便调用，可以使用pub use进行二次发布，方便用户使用
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;