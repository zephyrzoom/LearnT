/**
 * 执行cargo test来运行测试用例
 * 可以用cargo test -- 命令测试二进制
 * 多个用例是使用多线程并行执行的，应该确保他们直接没有依赖，或共享数据
 * cargo test -- --test-threads=1 这样可以设置关掉多线程
 * cargo test -- --nocapture 想要看到println的内容，关掉捕获
 * cargo test one_hundred 可以通过传入函数名为参数执行单个用例
 * cargo test add 用例中包含add的函数都会执行
 * cargo test -- --ignored 运行忽略掉的用例
 * 
 * 单元测试，通常在src目录的源码中加入tests模块，并添加cfg(test)注解
 */

#[cfg(test)]    // 告诉编译器执行cargo build的时候不执行这里面的代码，不要在编译时生成该代码的内容。ctg表示是配置文件，test是配置项，是rust提供的
mod tests { // 模块中可以有测试代码，也可以有非测试函数
    #[test] // 测试属性，说明这个函数用于测试。
    fn it_works() {
        assert_eq!(2 + 2, 4);   // 断言相等的宏
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test] // 失败的测试案例
    fn another() {
        panic!("Make this test fail");
    }

    use super::*;   // 引入模块外部的东西
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller)); // assert宏会验证结果是否为True，不是的话会调用panic宏
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));  // 验证相等，参数没有左右顺序。不等断言assert_ne!由于比较的类型必须实现PartialEq和Debug trait
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result // 添加第二个参数来设置报错信息
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 1000")] // 用例执行时应该panic，如果没有产生panic就会失败。使用expected参数后，参数中的字符串应是产生panic的字符串的子集，否则会失败
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test] // 使用Result代替assert时，不能用should_panic属性
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))   // 错误要返回Err对象
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);   // 测试的函数有println方法时，不会打印在终端
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore]   // 运行太慢的用例，想过滤掉用ignore属性
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));    // 私有函数同样可以直接调用测试，没有任何限制
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // 改成错的试试，测试案例就会报错，>改成<
        self.width < other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}