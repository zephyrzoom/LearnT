/**
 * Packages: 构建、测试和分发create用的
 * Crates: 一系列模块构成的库或可执行文件
 * Modules and use: 控制代码组成、作用域和私有path
 * Paths: 给一个结构体、函数或模块命名的工具
 * 
 * crate根目录有源码，编译器是从这里开始制作模块
 * package是一个或多个create，提供一组功能，用Cargo.toml文件描述
 * 
 * package必须有0个或1个library crate，他可以包含若干个crate(binary create或library crate)，但是至少要有一个
 * 如果有一个src/main.rs和一个src/lib.rs，那么他们会有和package相同的名字。一个package若有多个binary crate应该将文件放在src/bin目录，每个文件会拆分为一个binary crate
 * 
 * 每个crate中的变量函数都有自己的作用域和命名空间
 * 
 * use将path引入作用域，pub公有化声明
 */

use std::collections::HashMap;  // 引入struct就直接引入其本身就可以。std模块是rust语言默认附带的
use std::{cmp::Ordering, io};   // 可以同时引入多个
use std::io::{self, Write}; // 父级与子级同时引入，用self
use std::collections::*;    // 可以用*glob操作符，引入所有东西，不过使用时要很小心

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}