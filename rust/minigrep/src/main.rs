/**
 * 程序划分的准则是3条：
 * 1. 把代码分为lib.rs和main.rs，代码逻辑放在lib.rs
 * 2. 命令行处理简单的放在main.rs
 * 3. 命令行处理复杂的放在lib.rs
 */
use std::env; // 非unicode使用std::env::args_os
// use std::fs;
use std::process;
// use std::error::Error;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect(); // 接收命令行参数

    // let query = &args[1];
    // let filename = &args[2];

    // let (query, filename) = parse_config(&args);
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     // 闭包，就是匿名函数
    //     eprintln!("Problem parsing arguments: {}", err);    // 标准错误打印，错误信息会打印在终端，正确信息才会在文件中
    //     process::exit(1); // 终止程序并返回错误码
    // });

    let config = Config::new(env::args()).unwrap_or_else(|err| {    // args()返回的是iterator
        // 闭包，就是匿名函数
        eprintln!("Problem parsing arguments: {}", err);    // 标准错误打印，错误信息会打印在终端，正确信息才会在文件中
        process::exit(1); // 终止程序并返回错误码
    });

    // println!("Searching for {}", config.query);
    // println!("in file {}", config.filename);

    // let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}", content);

    // run(config);
    if let Err(e) = minigrep::run(config) {   // 正确时返回空，所以不需要unwrap正确返回，只需处理错误信息
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

// 使用结构体
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();    // 克隆会占用大量时间空间，尽量避免使用，这里为了简便所以使用
//     let filename = args[2].clone();
//     Config {query, filename}
// }

// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Config {query, filename}
//     }
// }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         // 用Result可以让main函数知道创建的情况
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();
//         Ok(Config { query, filename })
//     }
// }

// 把逻辑提取出来
// fn run(config: Config) {
//     let content =
//         fs::read_to_string(config.filename).expect("Something went wrong reading the file");
//     println!("With text:\n{}", content);
// }

// 返回错误信息
// fn run(config: Config) -> Result<(), Box<dyn Error>> {  // dyn意思是dynamic，可以返回实现Error trait的各种类型
//     let content = fs::read_to_string(config.filename)?;
//     println!("With text:\n{}", content);
//     Ok(())
// }
