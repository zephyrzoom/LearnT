fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);

    // 但是在同作用域中只能有一个易变引用，多了会产生数据竞争
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    {
        // 可以使用作用域操作符{}来创建多个易变引用
        let r1 = &mut s;
    }
    let r2 = &mut s;
    println!("{}, {}", r2, r2);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // 多个非易变引用是没有问题的，但是同时有不易变和易变引用就会有问题，除非在定义之后不再使用不易变引用
    // println!("{}, {}, and {}", r1, r2, r3);

    let reference_to_nothing = dangle();
}

// 使用引用不会转移所有权，s是指向s1的指针，引用参数称为借用，借用的变量是不能修改的
fn calculate_length(s: &String) -> usize {
    s.len()
}   // 因为没有获取所有权，所以最后不会drop堆内存

// 想修改引用，可以使用&mut修饰
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");
    // &s   // 如果返回引用，那么引用会被挂起。函数结束时s会被释放，但却要返回它的引用，引哪去啊
    s
}