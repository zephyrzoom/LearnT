// cargo test会执行所有crate的测试用例，cargo test -p add-one这样可以执行某个crate的

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

