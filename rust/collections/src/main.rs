/**
 * collections可以存储任何类型，而且存储在堆中，可以任意调整大小，但是他们的开销不同，使用时需要根据经验来选择
 */

fn main() {
    // Vec可以存相同类型的多个数据，Vec<T>可以存储任意类型，指定T后就只能存特定类型的值
    let v: Vec<i32> = Vec::new();

    let mut v = Vec::new(); // 这里也没有指定类型，会在存储时推断
    v.push(5);  // 添加数据用push，要添加的话v要设置成mut

    let v = vec![1, 2, 3];  // 通常不需要指定类型，用vec!会自动推断，推断出类型是Vec<i32>。当执行到作用域外时，v和他存储的内容都被释放

    let third: &i32 = &v[2];    // 可以用下标访问，用&配合[]获取引用
    println!("The third element is {}", third);

    match v.get(2) {    // get会返回Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let does_not_exist = &v[100];   // 会引起panic，程序会crash
    let does_not_exist = v.get(100);    // 会返回None，程序可以继续


    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];  // 不易变借用
    v.push(6);  // 易变借用，不易变和易变不能同时存在。这是因为，改变vec时可能导致内存重新分配，旧的数据会拷贝到新的内存空间，那么之前的借用就会保留已被释放的内存空间，造成内存泄漏
    println!("The first element is: {}", first);


    let v = vec![100, 32, 57];
    for i in &v {   // 遍历
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;   // 遍历带修改，需要先解引用再操作值
    }

    // 想要在vec存储不同类型的数据，可以借助enum，enum会被当做同种类型。编译时vec的类型都已经确定下来了，不知道类型需要使用trait对象
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}