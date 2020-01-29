/**
 * error分两种，一种是可恢复的，一种是不可恢复的，可恢复的比如文件找不到，不可恢复的就是bug
 * 其他语言一般是不区分的都用exceptions，Rust对可恢复错误用Result<T, E>，对不可恢复的用panic!宏
 *
 * 到底使用哪种错误：
 * 因为panic代表没有机会恢复，而Result则是可以恢复的，也是可以转换成panic的，所以默认建议使用可恢复的
 * 在一些原型代码和测试中适合使用不可恢复错误，在原型代码中还没考虑好要如何错误处理时，或者测试代码出错就想全部结束，那么用unwrap或expect调用panic!就很合适
 * let home: IpAddr = "127.0.0.1".parse().unwrap();    // 这里使用unwrap就很合适，因为ip地址已经硬编码了，肯定是对的，并不会报错
 * 一些坏的情况发生时适合使用panic，设定的一些假设，不变内容被打破时，如不合法的值、矛盾的值或丢失值
 * 别人调用你的库时，你认为他的使用方式不正确，给他返回panic已警示他修改；或者调用别人的库时，存在无法修复的错误，你也可以返回panic
 * 通常处理数据时先验证数据有效性，无效就panic，可以减少漏洞，自己的代码产生panic的地方应该写明原因
 * Rust提供了类型验证机制，来对Result过多match提供便利
 */
use std::fs::File;
fn main() {
    panic!("crash and burn"); // 主动引起一个错误，运行会打印出错误字符串。有些错误可能是调用的其他函数触发的，用backtrace来追溯

    let v = vec![1, 2, 3];
    v[99]; // vec会报错panic，超出了集合的大小。而C语言就不会，他会访问超过范围的内存地址，导致安全漏洞

    // 通过在运行的时候设置RUST_BACKTRACE，可以看到调用链的函数，究竟是从哪里引发的这个错误
    // 将RUST_BACKTRACE设置为不是0的数
    // 错误信息需要打开调试模式才能看到，不带--release的cargo build和cargo run都可以

    // enum Result<T, E> {  // Result的定义是这样的，有两个参数
    //     Ok(T),   // Ok时返回T类型参数
    //     Err(E),  // Err时返回E类型参数
    // }

    let f: u32 = File::open("hello.txt"); // 写一个错误的返回类型可以看出报错信息中提示的实际返回类型std::result::Result<std::fs::File, std::io::Error>。open执行正确则会得到文件的对象，否则会得到错误信息

    let f = File::open("hello.txt");
    let f = match f {
        // 用match来处理结果
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 不同错误区别处理，error对象附带的类型信息io::ErrorKind
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 老司机都这样写，删掉过多的match可以让代码更整洁
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let f = File::open("hello.txt").unwrap(); // unwrap可以简化match，Ok时会返回Ok内的对象，Err时会执行panic!

    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // 类似unwrap，不过他可以返回错误信息
    
}

use std::io;
use std::io::Read;
// 返回错误信息，使错误传递给调用者，让他去决定如何处理
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 用?更简单的实现
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // 在Result类型后跟?相对与match语句，匹配返回Ok的值，不匹配直接return错误。但是和match有些区别，错误实现了From特性的话，会自动调用from函数转换类型
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 更加简化的方式，可以连续?
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;
use std::io;
// 最简化的方法已被封装在fs中了
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let f = File::open("hello.txt")?; // 在main函数使用会报错，main函数特殊，会返回()
}

fn main() -> Result<(), Box<dyn Error>> {
    // 用这种方式就可以，Box表示接受任何错误
    let f = File::open("hello.txt")?;
    Ok(())
}

// 可以用这种方式到处对输入的guess进行范围验证，会出现很多冗余代码，应该将其封装在一个类型中
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
    // --snip--
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value   // 范围正确则用value域创建guess对象
        }
    }

    // 就是俗称的get方法
    pub fn value(&self) -> i32 {
        self.value
    }
}