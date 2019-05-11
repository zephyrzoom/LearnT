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