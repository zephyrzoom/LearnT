// 所有权是rust特有的，他保证了无gc情况下的内存安全
// 其他语言不用考虑内存是分配在堆还是栈中，而在rust需要考虑
// 栈要求固定大小的数据，堆则是变化的大小或者运行时才知道大小
// 在栈中分配内存比堆快，因为堆的话要操作系统搜索一片空内存
// 取数据的时候也是从栈取比从堆取快
// 函数实参是在堆上的，本地变量在栈上
// 所有权就是为了管理堆的
// 所有权三原则：
//  1. 每个值的变量是他的所有者
//  2. 同时只能有一个所有者
//  3. 所有者超出作用域时，变量的值会被丢弃
fn main() {
    // 在{}范围内都是该变量作用域，在其中是有效的
    let s = "hello";

    // 字符串字面值是需要提前知道的，而String不需要，String是分配在堆中的
    // ::用来访问命名空间下的方法
    let s = String::from("hello");

    // 字符串可以修改值
    // 调用from时向系统的堆中请求内存，在作用域结束时会自动归还内存，就是会自动调用drop函数
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // 5绑定到x
    let x = 5;
    // 复制x的值再绑定到y，这些简单类型都是在栈上分配的
    let y = x;
    // 了解了String再来看这里，x和y的值都在栈中，拷贝很快，不会move所有权，都有效
    println!("x = {}, y = {}", x, y);

    // 这里和上面就不同了，s1字符串由三部分组成，指向字符串内容的指针、长度和容量，都是在栈上申请的，而右边的字符串内容是从堆中申请的
    // s1                   
    // |name    |value|     |index|value|
    // |--------|-----|     |-----|-----|
    // |ptr     |  ---|---->|0    |h    |
    // |len     |5    |     |1    |e    |
    // |capacity|5    |     |2    |l    |
    // |--------|-----|     |3    |l    |
    //                      |4    |o    |
    //                      |-----|-----|
    let s1 = String::from("hello");
    // s2只复制了左边的s1，堆中的内容没有复制，所以s1和s2都指向同一片堆内存
    // 如果按照之前的说法，作用域结束之后会释放两次堆内存
    let s2 = s1;
    // 因此，rust会将s1失效，print会报错。这种操作叫做move，相当于把所有权交给了s2，rust中不存在深拷贝
    // rust有Copy特性，使用这个就不会让就旧变量失效，但是该特性不能和Drop特性同时使用，Drop中可能有其他释放操作
    // 有Copy特性的类型有：整型、bool、浮点、char和包含以上类型的tuple
    // println!("{}, world!", s1);

    let s1 = String::from("hello");
    // 深拷贝
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");  // s在当前作用域有所有权
    takes_ownership(s);             // 函数传值相当于赋值，所有权会move
    // 现在s就不能用了
    // println!("{}", s);
    let x = 5;                      // x在当前作用域有所有权
    makes_copy(x);                  // x仍有效，他有Copy特性
    println!("{}", x);

    let s1 = gives_ownership(); // s1获取所有权
    let s2 = String::from("hello"); // s2有所有权
    let s3 = takes_and_gives_back(s2);  // 所有权转一圈又回来了，s2的所有权交到了s3手里

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);   // 可以通过tuple返回多个值
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}