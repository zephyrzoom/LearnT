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
 * 
 * RefCell使用了内部易变的设计模式，可以在有不易变引用的情况下易变数据，在数据结构内部使用unsafe代码来折中borrow规则，borrow规则会在运行时保证
 * RefCell只有单所有权
 * borrow规则是这样两条：
 * 1. 可以有一个易变引用或者多个不易变引用
 * 2. 引用必须是有效的
 * Box<T>是在编译期保证borrow规则的，而RefCell是在运行时
 * 编译器的静态分析能力是有限的，RefCell就可以在用户确定可以遵循borrow规则而编译器无法确定时使用
 * RefCell和Rc都是在单线程使用的
 * ReCell会在运行期检查borrow规则，不符合会panic
 * 智能指针的选择：
 * 1. Rc允许有多所有者，Box和RefCell都是单所有者
 * 2. Box允许在编译期使用变异或不易变的borrow，Rc允许在编译期使用不易变的borrow，RefCell在运行时允许不易变或易变borrow
 * 3. RefCell允许在运行时易变借用，你可以在RefCell是不易变时，在内部改变值
 * 
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

    // 这样是不行的
    let x = 5;
    let y = &mut x;

    // 
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


#[derive(Debug)]
enum List {
    // 使用Rc和RefCell可以让链表的多个引用都可以改变指向的值
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));   // value拥有5的所有权
    // clone可以不转移所有权情况下获得所有权
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // b和c都指向a，有a的所有权
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    // value自动解引用获得RefCell，再borrow_mut获得RefMut，最后*来解引用RefMut引用修改值
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// rust允许使用Rc和RefCell能够造成内存泄露
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// 定义循环引用，b指向a，a再指向b
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    // b指向a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // a再指向b，a.tail拿到RefCell<Rc<List>>
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // 会导致栈溢出
    // println!("a next item = {:?}", a.tail());
}   // 最终内存不会得到释放，引用循环是逻辑错误，应该用其他手段避免

/**
 * 可以使用Weak<T>来避免引用循环
 * Rc::clone增加strong_count，Rc::downgrade增加weak_count，会得到一个Weak<T>智能指针，weak_count不为0时也可以释放内存
 * Weak引用不会表达所有权的关系，不会引起引用循环，强引用为0时就会破坏循环
 * Weak引用不知道自己是不是有效的，所以要调用upgrade来返回Option<Rc<T>>来判断
 */
// 构造一个树型结构，这种结构只能知道父节点的子节点，而不知道子节点的父节点是谁
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    // 叶节点
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]), // 指向叶节点
    });
}

use std::rc::{Rc, Weak};
use std::cell::RefCell;
// 父节点删除子节点应该一起删除，子节点删除父节点并不删除，这就是Weak引用。仅仅指向而不拥有
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,    // 为了子节点能够知道父节点，需要这个属性，但是不用Rc，Rc会造成循环引用
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),  // 初始化为空的
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // none

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 用downgrade建立一个弱引用
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // Some变体
}

// 区分strong_count和weak_count
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),    // 1：leaf
        Rc::weak_count(&leaf),  // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),  // 1：branch
            Rc::weak_count(&branch),    // 1：leaf
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),    // 2：leaf、branch
            Rc::weak_count(&leaf),  // 0
        );
    }   // branch到作用域结束，strong_count被置0，leaf变量对branch的弱引用不会影响到它的释放。这时leaf的强引用变为1

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
