/**
 * 智能指针可以拥有多个所有者，并且会跟着所有者的数量，没有所有者时会清除数据
 * 引用与智能指针的区别：引用是只借用数据的指针，智能指针拥有他们指向的数据
 * String和Vec<T>就使用了智能指针
 * 智能指针都是结构体，实现了Deref和Drop trait，Deref使得使用他可以像使用引用一样，Drop使得你可以自定义代码执行完作用域时的行为
 * Box<T>在堆中分配数据
 * Rc<T>允许多所有者的引用计数类型
 * Ref<T>和RefMut<T>允许通过RefCell<T>强制在运行时借用而非编译时
 * 
 * Box在这三种情况下使用：
 * 1. 编译时不知道类型的大小，但是在上下文使用时却需要确定的大小
 * 2. 有大量数据想转移所有权，但是不希望数据拷贝
 * 3. 想要拥有一个实现了某个trait的值的所有权
 * 
 * Deref针对不易变，DerefMut针对易变
 * 
 * 三种情况会强制解引用：
 * 1. 当T: Deref<Target=U>时，从&T到&U
 * 2. 当T: DerefMut<Target=U>时，从&mut T到&mut U
 * 3. 当T: Deref<Target=U>时，从&mut T到&U
 * 第三条只能mut转immutable，因为mut引用一定是唯一的引用，若有immutable转mut则，不能保证唯一，转换过程中不会检查引用个数
 * 
 * Rc<T>就是reference count，在堆中分配的数据，给多个部分使用，不知道谁用到最后，这种情况就要用Rc<T>
 */

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::ops::Deref;

fn main() {
    let b = Box::new(5);    // 这个5存储在堆中，作用域结束时会释放栈中的box，就是b和堆中的5
    println!("b = {}", b);

    let list = Cons(1, Cons(2, Cons(3, Nil)));

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)))))); // 使用Box


    let x = 5;
    let y = &x; // 引用
    assert_eq!(5, x);
    assert_eq!(5, *y);  // 解引用
    assert_eq!(5, y);   // 这样会报错，不同类型无法比较


    let x = 5;
    let y = Box::new(x);    // 用box定义后，y的使用和引用一样
    assert_eq!(5, x);
    assert_eq!(5, *y);


    let x = 5;
    let y = MyBox::new(x);  // 使用自己定义的box
    assert_eq!(5, x);
    assert_eq!(5, *y);  // 必须实现Deref trait。*y实际上是*(y.deref())，相比而言引用则实际上是*(&y)

    // 隐式强制解引用
    let m = MyBox::new(String::from("Rust"));
    hello(&m);  // 由于MyBox实现了deref，所以&MyBox<String>会被转为&String，而String也实现了deref，所以会再次强制转换为&str。deref操作都是在编译器完成的，不会造成运行时损失

    // 如果没有强制解引用机制，则要这样调用
    hello(&(*m)[..]);   // *m把MyBox<String>转换为String，再获取其切片

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");   // 出作用于后，就会销毁变量，会以和创建顺序相反的顺序销毁

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    c.drop();   // drop是不允许手动显示调用的，是由智能指针自动管理的。如果我们也drop一次，最后还会drop，就引起了多次析构bug
    println!("CustomSmartPointer dropped before the end of main.");

    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);    // 可以调用std::mem::drop函数来显示析构
    println!("CustomSmartPointer dropped before the end of main.");


    // 创建b和c都指向a的链表，会报错
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));   // 这里a的所有权给了b
    let c = Cons(4, Box::new(a));   // 这里a没有所有权给c，所以报错


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a)); // 调用clone就会多一次引用
    let c = Cons(4, Rc::clone(&a)); // 调用a.clone()也可以，但是不会深度拷贝，只会增加引用次数，强烈建议使用Rc::clone便于区分


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));  // strong_count可以查看引用的次数
    let b = Cons(3, Rc::clone(&a)); // 调用clone就增加一次
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }   // 作用于结束引用会减少一次，Rc实现了Drop trait，会自动调用来减少引用次数
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // Rc只能用在immutable，如果用于mut违反borrow规则，也会造成数据竞争
}

enum List {
    Cons(i32, List),    // 编译失败，不知道类型的实际大小，枚举的大小是根据它的最大变体确定的，但是递归计算就算不出具体大小
    Nil,
}

enum List {
    Cons(i32, Box<List>),   // Box<T>的大小是固定的，这样就可以确定大小了
    Nil,
}

// 自己实现的box，结构体元组
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;    // 定义关联的类型

    // 参数是borrow self，返回类型的引用，大部分情况我们是不希望拿走self的所有权
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


struct CustomSmartPointer {
    data: String,
}
// 实现Drop使得只能指针自动销毁变量
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// 使用Rc<T>多次引用
enum List {
    Cons(i32, Rc<List>),
    Nil,
}