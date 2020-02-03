/**
 * 可以执行cargo run -p adder执行本crate
 */

use add_on;
use rand;
fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, add_on::add_one(num));
}
