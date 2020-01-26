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
        _ => (),    // 其他情况都在这里匹配，()是单位值，什么也不做
    }

    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // 与上面等价，if let用来简化书写
    if let Some(2) = some_u8_value {
        println!("three");  // 这里匹配就不会进行完备检查，其他情况有可能漏掉。只有需要匹配一种模式，其他都不需要时建议使用
    }


    let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count: {}", count);

    // 与上面的等价， if let可以和else搭配使用else扮演了_的角色
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
    Quarter(UsState),   // 可以给枚举值设置另一个枚举类型
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {    // match匹配的是coin类型，匹配的时候依次进行，匹配上的话会执行=>右侧的代码，每个分支右侧是表达式
        Coin::Penny => {    // 简单代码可以用一行，多行的话需要用大括号
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,  // 每个匹配分支都由“模式=>执行内容”构成，分支之间用,分隔
        Coin::Dime => 10,
        Coin::Quarter(state) => {   // 当coin为Coin::Quarter(UsState::Alaska)时会匹配这个分支，也可以获得state绑定的枚举值
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {    // Option枚举类型也可以用来匹配
    match x {
        None => None,   // 返回空，不写对None值的匹配分支，会报错，匹配的分支覆盖不完整
        Some(i) => Some(i + 1), // 匹配会给值+1
    }   // 当参数是Some(5)时，先匹配None，不匹配；再匹配Some(i)时，有相同变体匹配成功，i会赋值5，然后执行分支体，创建一个新的Some
}
