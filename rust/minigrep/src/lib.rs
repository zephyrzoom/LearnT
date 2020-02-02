use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     // 用Result可以让main函数知道创建的情况
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 如果CASE_INSENSITIVE设置了，就会返回false
    //     Ok(Config { query, filename, case_sensitive })
    // }

    // env::args()返回的是Args迭代器，要遍历args所以设置为mut
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();    // 跳过第一个迭代器值

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {  // dyn意思是dynamic，可以返回实现Error trait的各种类型
//     let content = fs::read_to_string(config.filename)?;
//     println!("With text:\n{}", content);
//     Ok(())
// }

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {  // dyn意思是dynamic，可以返回实现Error trait的各种类型
    let contents = fs::read_to_string(config.filename)?;
    // for line in search(&config.query, &content) {
    //     println!("{}", line);
    // }

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// 需要给出生命周期
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }

// 用迭代器简化搜索
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();   // 该方法创建了新字符串，而不是之前的切片引用
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}