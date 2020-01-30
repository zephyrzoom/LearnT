pub trait Summary {
    // trait后面是名字
    fn summarize(&self) -> String; // 每个方法声明写一行，用分号结束，可以有多个方法
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 定义方法是在impl后写trait名，再在其后用for和要实现的类型
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // 这里和正常的函数实现一样
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

use aggregator::Summary; // 要想实现其他crate的trait方法，需要先用use引入，再去定义

// 可以定义默认实现，就不用给每个类都实现了
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl Summary for NewsArticle {} // 不用实现就可以

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // trait中的默认方法甚至可以调用其他还没实现的方法
    }
}
// 只用实现summarize_author就够了
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// 可以使用实现了Summary trait的类型作为参数，可以传入NewsArticle或者Tweet
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 与上面的定义相同，只是更冗余，用到了泛型trait限定，限定在实现了Summary trait的类型上
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

// 也可以使用两个trait作为参数
// pub fn notify(item1: impl Summary, item2: impl Summary) {

// 如果想让两个trait参数类型一致，那么就要使用泛型trait限定，都指定为T
// pub fn notify<T: Summary>(item1: T, item2: T) {

// 可以指定多个trait限定，参数必须是实现了两个trait的类型
// pub fn notify(item: impl Summary + Display) {

// 多个trait限定也可以用在泛型格式中
// pub fn notify<T: Summary + Display>(item: T) {

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// 多个trait限定导致函数可读性差，使用where关键字
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {

// 返回类型也可以用trait，返回类型是实现了Summary trait的类型，但是调用者不知道具体返回的是什么类型。返回trait类型，一般用在闭包或迭代器
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 这样写返回两个trait类型是不行的，编译器不允许
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}