/**
 * 泛型属于单态化的，在编译时就能进行静态分发，指派方法
 * 而trait则是在运行时动态分发的，会有一些查找的性能损耗，但是代码写法更灵活
 * 
 * 类型安全的trait对象有两个标准，trait的所有方法都：
 * 1. 不返回Self
 * 2. 无泛型参数
 * Self表示了实现的类型，一旦使用了trait对象，就不知道trait的实现类型，不知道实现类型却要返回；泛型是类似的
 */
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use gui::{Screen, Button};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No")
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // run会调用draw方法
    screen.run();
}

use gui::Screen;

fn main() {
    let screen = Screen {
        components: vec![
            // 编译会报错，因为String没有实现Draw trait
            Box::new(String::from("Hi")),
        ],
    };

    screen.run();
}


/////////////////////////////////////////////////////
/// 实现state模式
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

/////////////////////////
/// 最终的main，中间状态不能调用content，会报错，这样在编译期就能够发现错误
/// 使用类型系统硬编码状态
use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}