/**
 * 默认都是私有的，加了pub才是公有
 * 结构体是pub的，但是它的字段仍然是私有
 * 
 * RUST没有继承机制，但是有trait，有类型限制的trait会形成有种有限类型下的多态
 */
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

/**
 * 三个公有方法操作数据
 */
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // 通过这个私有方法修改数据
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

////////////////////////////////////////////////////////////
/**
 * 多态的例子
 * trait对象类似传统的对象，但是无法添加数据
 */
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // Box中是任何实现了Draw trait的trait对象，Vec中可以是不同类型的对象
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 使用泛型是达不到上面的效果的，泛型在运行时只能有一种类型，而trait可以是多种类型
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 定义GUI的按钮类型
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}


pub struct Screen {
    // 这里会报错，Clone trait的方法会返回Self导致不安全
    pub components: Vec<Box<dyn Clone>>,
}

//////////////////////////////////////////////////
/// Post定义
/// 如果不使用state模式，就需要写很多match语句，而且要了解每种状态的情况
pub struct Post {
    // 状态trait实现多态
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            // post肯定是从Draft状态开始
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    // 与state模式无关
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // Draft状态返回空
    pub fn content(&self) -> &str {
        // as_ref会返回Option<&Box<dyn State>>
        self.state.as_ref().unwrap().content(self)
    }
    // 先把state字段置None，再设置新state；没有直接设置self.state = self.state.request_review()
    // 为了夺取旧值所有权，使其不可用
    pub fn request_review(&mut self) {
        // take会把state字段值取出，并把state字段置None
        if let Some(s) = self.state.take() {
            // 切换状态
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // 参数用Box会夺走所有权，使旧的状态失效
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    // Draft状态调用不起作用，所以返回self
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // 返回自己即可，状态已经在PendingReview状态了
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // 改变状态
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

///////////////////////////////////////////////////
/// DraftPost不定义content方法
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    // 此时返回DraftPost类型，也就无法调用content，没有Post对象
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    // 执行后原来的DraftPost对象就释放了，因为入参是self
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

// 同样没有content方法
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}