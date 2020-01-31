/**
 * 集成测试代码放在tests目录下，也不需要cfg(test)注解
 * 执行crago test时会执行单元测试、集成测试和文档测试
 * cargo test --test integration_test 指定执行的用例文件
 * 集成测试代码相当于一个新的crate，可以用mod来拆分管理
 */

use tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, tests::add_two(2));
}
