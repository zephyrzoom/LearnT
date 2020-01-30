/**
 * 生命周期大部分是隐式的和被推断的
 */

fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;
        }   // 这里执行完，作用域外时x被释放了，r引用的x造成了引用悬挂，x不能再借用了
        println!("r: {}", r);
    }

    // 标注了生命周期，编译器会进行借用检查，r的生命周期'a范围大于x的生命周期'b，但'a却引用了'b的内存，该操作会被拒绝
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+


    // 这样修改之后就好了，x的生命周期大于r，r去引用x就没问题
    {
        let x = 5;            // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    // 两个引用可以有相同的生命周期参数，说明在该生命周期中他们必须都是可用的
    &i32        // 无生命周期参数的引用
    &'a i32     // 有生命周期参数a的引用
    &'a mut i32 // 有生命周期参数a的易变引用


    // 这是另一种需要缩小生命周期的情况
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };  // i的生命周期不能超过part的生命周期

    // 静态生命周期，字符串是硬编码在代码里的，是一直可用的。给引用使用是要想清楚它的生命周期是不是一直存在
    let s: &'static str = "I have a static lifetime.";
}

// 由于无法确定到底返回x还是y，所以无法确定他们的生命周期，所以编译器会报错。使用泛型生命周期参数来解决
fn longest(x: &str, y: &str) -> &str {  // 应用规则1:fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    if x.len() > y.len() {              // 规则2无法使用，他是函数，不是方法，没有self参数
        x
    } else {
        y
    }
}

// 添加泛型生命周期参数，所有的引用和返回值都有相同的生命周期
// 他们告诉了编译器要检查参数是不是至少在a生命周期内存活，生命周期参数没有改变任何值，至少告诉编译器的借用检查器要检查是否满足这个约束
// 生命周期注解都在函数生命中添加，不在函数体。当有引用入参或者调用其他函数时，生命周期无法推断，就需要人为指定
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 函数被调用时，生命周期a会替换成实际参数的生命周期
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 始终返回x的话，y就不需要生命周期注解
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 返回值必须和入参的某一个参数生命周期一直，不然就会产生引用悬挂。可以通过返回所有权而不是引用来解决
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// 结构体也可以用引用类型，也可以设置生命周期参数
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 这个函数不需要显示指定生命周期，有些情况编译器可以通过推测确定，无法确定的就需要手动指明
// 入参称作输入生命周期，返回值称作输出生命周期
// 编译器会根据三大准则分析，满足就可以进行生命周期消除，第一个针对输入生命周期，后两个针对输出生命周期，使用在fn和impl定义
// 1.每个参数都有自己的输入生命周期
// 2.只有一个输入生命周期，就会赋给输出生命周期
// 3.如果有多个输入生命周期，其中一个参数是&self或&mut self，那么输出生命周期就是self的生命周期
fn first_word(s: &str) -> &str {    // 使用规则1:fn first_word<'a>(s: &'a str) -> &str {
    let bytes = s.as_bytes();       // 使用规则2:fn first_word<'a>(s: &'a str) -> &'a str {
                                    // 应用规则2后所有的引用都有生命周期了
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExcerpt<'a> { // 这里必须加生命周期参数
    fn level(&self) -> i32 {    // 这里使用规则1
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {    // 使用规则1和规则3
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 使用了泛型、trait限定和生命周期注解的复杂函数定义
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}