// 使用闭包和迭代器没有性能损耗，推荐使用
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();    // 什么也没做

    // for循环不需要定义v1_iter为mut，循环会直接获取所有权
    for val in v1_iter {    // iter省去了用下标递增去遍历数据的方式，简化代码
        println!("Got: {}", val);
    }

    let v1: Vec<i32> = vec![1, 2, 3];
    // map叫迭代器适配器，可以产生新的迭代器
    v1.iter().map(|x| x + 1);   // 不会执行会生成警告，必须要调用迭代器消耗方法，才会触发执行


    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();    // collect会消耗迭代器，所以会执行
    assert_eq!(v2, vec![2, 3, 4]);
    
}

// 每个迭代器都实现了Iterator trait，这是标准库的定义
pub trait Iterator {
    type Item;  // 定义一个与本trait关联的类型

    fn next(&mut self) -> Option<Self::Item>;   // 调用next时返回Item类型。迭代器只用实现next函数就可以，会返回Some或None

    // methods with default implementations elided
}