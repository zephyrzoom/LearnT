fn main() {
    // 切片数据没有所有权，可以建立切片的引用
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("the first word is: {}", word);
    s.clear();
    // println!("the first word is: {}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];   // 引用字符串的一部分，0,1,2,3,4,
    let world = &s[6..11];

    let s = String::from("hello world");
    let hello = &s[0..=4];
    let world = &s[6..=10];

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];    // 和上面的相等

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];    // 和上面的相等

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];     // 和上面的相等

    // s是&str类型，字符串字面值是不易变的，&str也是不易变的
    let s = "Hello, world!";

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    // 这个切片是&[i32]类型，和字符串切片类似
    let slice = &a[1..3];
}

// 没有获取字符串所有权，只能通过返回结束下标达到目的，不过这种方式是有问题的，返回的位置数据可能由于字符串的改变而失效
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();    // 将字符串转换为字节数组匹配
       // iter返回数组的每个元素，enumerate包装每个元素为元组，元组格式（序号，元素引用）
//     for (i, &item) in bytes.iter().enumerate() { 
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// 这种写法的好处是会让s.clear报错，因为有个不易变引用了，再用易变引用修改值就是不允许的
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// 入参也是&str，那么就可以传String转成的str，也可以传String的切片
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}