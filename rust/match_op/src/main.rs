// match可以匹配各种类型模式
fn main() {
    let penny = Coin::Penny;
    println!("coin value is {}", value_in_cents(penny));
    println!("coin value is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 2u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 与上面等价
    if let Some(2) = some_u8_value {
        println!("three");
    }


    let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count: {}", count);

    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {    // match的是coin类型，匹配的时候依次进行，匹配上的话会执行=>右侧的代码，每个分支右侧是表达式
        Coin::Penny => {    // 简单代码可以用一行，多行的话需要用大括号
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,  // 每个匹配分支都由“模式=>执行内容”构成，分支之间用,分隔
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
