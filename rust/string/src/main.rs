/**
 * Rust中实际只有一种字符串类型就是str，和切片类型的&str
 * String则是标准库中的，growable, mutable, owned三个特性
 * 通常说的字符串指的是String和&str，都是UTF8编码的
 * 其他字符串类型：OsString, OsStr, CString, and CStr
 */

fn main() {
    let mut s = String::new();  // 用new创建String

    let data = "initial contents";
    let s = data.to_string();   // to_string将字符串转换为String类型

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");   // 直接用字符串创建String

    // utf-8的所以下面这些都合法
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");  // s拼接成foobar，push_str参数是&str，字符串切片&str，不会得到所有权

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);    // 不会夺走s2的所有权，后面还可以打印
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');    // 添加单字符

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1的所有权移动了，后面将无法再使用，s2没有被移动。加法函数是这样的fn add(self, s: &str) -> String {。按理说s2应该是&str才能符合参数类型，但是在这里却可以执行，因为&s2会被转换为&s2[..]。add第一个参数是self，不是&self，所以s1会移动所有权，只有s2的内容被拷贝。

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // 不推荐这样拼接
    let s = format!("{}-{}-{}", s1, s2, s3);    // 用format!宏拼接更好，他不会挪走任何参数的所有权

    let s1 = String::from("hello");
    // let h = s1[0];   // 不能用下标索引字符串

    // String类型是对Vec<u8>的包装
    let len = String::from("Hola").len();
    // 俄语的长度是24，因为这些字符串用utf8编码时是变长的，占的字节数不一样，一个字符两个字节
    let len = String::from("Здравствуйте").len();

    let hello = "Здравствуйте";
    let answer = &hello[0]; // 感觉是应该返回З，但是З编码为两个字节，208和151，取出来就是208，但是208无法对应З，所以不能用下标索引字符串

    let s = &hello[0..4];   // 知道一个字符占两个字节，前四个字节就是前两个字符，就可以用范围切片来获取字符串，如果范围不对就会panic

    for c in "नमस्ते".chars() {   // 这个字符串存储的字节是[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
        //  unicode scaler存储的是['न', 'म', 'स', '्', 'त', 'े']，其中多了两个读音符号。chars()方法可以获取unicode字符
        // 字素应该是4个["न", "म", "स्", "ते"]
        // 因为这种字符的长短不一，所以Rust没有选择用下标索引，无法保证O(1)的时间复杂度
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {   // 获取字符串的字节
        println!("{}", b);
    }
}
