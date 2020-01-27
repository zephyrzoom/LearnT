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


mod sound;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::sound::instrument::woodwind;
// use self::sound::instrument::woodwind;

mod performance_group {
    // use引入后的模块为私有模块
    pub use crate::sound::instrument::woodwind;

    pub fn clarinet_trio() {
        woodwind::clarinet();
        woodwind::clarinet();
        woodwind::clarinet();
    }
}

use std::collections::HashMap;
use rand::Rng;
fn main() {
    // Absolute path
    crate::sound::instrument::woodwind::clarinet();

    // Relative path
    sound::instrument::woodwind::clarinet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    woodwind::clarinet();

    performance_group::clarinet_trio();

    let mut map = HashMap::new();
    map.insert(1, 2);

    performance_group::woodwind::clarinet();

    let secret_number = rand::thread_rng().gen_range(1, 101);
}

use std::{fmt::Result, io::Result as IoResult};
use std::io::{self, Write};
use std::collections::*;
// fn function1() -> Result {
// }
// fn function2() -> IoResult<()> {
// }