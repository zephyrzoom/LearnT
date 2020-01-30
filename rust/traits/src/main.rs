/**
 * trait用来定义共享的行为，类似于其他语言的接口。
 * 不同的类型想要共享相同的行为，trait可以将函数聚集在一起，提供一组行为操作
 * 实现trait的方法时，只能实现trait定义在本crate或者类型定义在本crate的情况，不能在外部的类型实现外部的trait。按照相关性，还有孤儿法则，就是不同crate的代码互相不影响，如果不遵守，怎么大家都可以定义外部类型的外部trait，编译器就不知道应该调用哪一个。
 */

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    // 实现了summarize trait的类都可以调用他
    println!("1 new tweet: {}", tweet.summarize());
}

// 用trait修改largest的bug，限定泛型实现PartialOrd trait
fn largest<T: PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];  // 这里会报错，因为可能有类型没有实现copy trait，之前是i32和char类型都存在栈中，实现了Copy trait

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 通过限定PartialOrd和Copy trait可以消除报错。还可以用Clone限定代替Copy，只是在堆克隆要慢一些
// 其实还有另一种方法，可以将返回类型改为&T，就不用实现Copy或者Clone了，还能避免在堆分配数据
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {    // Pair<T>通常实现的是new方法
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) { // 如果Pair<T>是PartialOrd限定的就只用实现cmp_display即可
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// 称为地毯实现，在任何实现了Display的类型中都可以调用ToString的方法
impl<T: Display> ToString for T {
    // --snip--
}