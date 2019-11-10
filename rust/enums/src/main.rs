fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 可以调用任一个，::就可以访问
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    // Option是一个标准的enum
    // rust中是没有null的，null造成了很多漏洞，在认为他是非null的时候调用他的方法
    // 定义None的时候要指定Option的类型，无法自动推断该类型
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 不同类型会报错，i8和Option<i8>是不同类型
    // 这样就不会错误的使用None，不过使用时要把Option<T>转换成T
    // let sum = x + y;
}

// 枚举同时只有一种情况，这里使用正合适，不管是V4还是V6都是同种类型
enum IpAddrKind {
    V4,
    V6,
}

// 传入这个类型就够了
fn route(ip_type:IpAddrKind) {}

// 可以用IpAddrKind定义结构体字段
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// 可以给枚举设置关联值的类型，一个或多个都可以
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 枚举可以存储各种类型
enum Message {
    Quit,   // 不设置类型
    Move { x: i32, y: i32 },    // 匿名结构体
    Write(String),  // 一个字符串
    ChangeColor(i32, i32, i32), // 三个数字
}

// 用struct也可以定义上面的东西，但是使用起来只能独自用
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// 枚举类型也可以定义方法
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// 标准库里是这样定义的，虽然rust没有null，但是可以直接使用这个枚举表示，它被预先都引入了，无须Option::Some，可以直接Some或None
// enum Option<T> {
//     Some(T),
//     None,
// }