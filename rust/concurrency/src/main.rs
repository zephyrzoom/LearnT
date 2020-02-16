/**
 * 并发：程序的不同部分独立执行（Concurrent）
 * 并行：程序的不同部分同时执行（parallel）
 * 
 * 多线程会导致的问题：
 * 1. 资源竞争
 * 2. 死锁
 * 3. 无法复现的bug
 * 
 * 操作系统一般都提供线程创建，有的程序也会
 * 1. 1:1，一个程序线程对应一个操作系统线程
 * 2. M:N,绿色线程，M个程序线程对应N和操作系统线程
 * Rust采用1:1,开销少，不会增加运行时
 * 
 * 线程分享数据的口号：不通过共享内存交流，而是通过交流来共享内存
 * 共享数据用的channel，分两部分，一半是发射器，一半接收器，任何一半关闭，channel就会关闭
 * 
 * Rust语言只提供了std::marker的两个trait:Sync和Send，实现他们就可以实现自己的并发机制
 * 实现Send就可以在线程间传递，基本类型几乎都是Send，除了raw指针
 * 实现Sync可以在多个线程被引用，如果T类型是Sync，那么&T就是Send
 * Rc<T>不是Send而Arc<T>是，RefCell<T>不是Sync而Mutex<T>是
 * 实现了Send或Sync的类型就是Send或Sync类型，手动实现是unsafe的，实现时应确保safe
 */
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

fn main() {
    // 使用spawn和闭包创建线程
    let handle = thread::spawn(|| { // spawn会返回一个JoinHandle
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // handle.join().unwrap();  // 放在这里就先执行handle线程的再执行当前线程
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 调用join来等待线程执行完。阻塞当前线程直到handle的线程执行完


    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v);    // 如果有这条语句，子线程就不知道v是否还有效，所以要用move。但是move之后，失去了v的所有权，v不能被drop
    handle.join().unwrap();

    // 创建一个空channel，mpsc意思是multi producer single consumer，多发单收
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();  // send会返回Result<T, E>，如果管道关闭了就可以知道
        // println!("val is {}", val); // val所有权已经move了
    });

    // 还有一种try_recv方法，他立即执行，会返回OK里面带着数据，或者没有数据时返Err
    let received = rx.recv().unwrap();  // recv会阻塞线程，一直接收数据直到通道关闭
    println!("Got: {}", received);


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();  // 发送多个值
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }


    let (tx, rx) = mpsc::channel();
    // 克隆一个发射器，给另一个线程
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    /**
     * channel属于单所有者，共享内存相当于多所有者
     * Mutex是mutual exclusion的缩写，任何时刻只能有一个线程有权限，用之前要上锁
     * 有两条规则：
     * 1. 用前必须上锁
     * 2. 用完之后要解锁
     * Mutex使用时经常会出错，而Rust的类型规则和所有权规则可以避免出错
     * Mutex<T>之于Arc<T>相当于RefCell<T>之于Rc<T>
     */
    // Mutex<T>创建
    let m = Mutex::new(5);

    {
        // Mutex其实是一个智能指针，调用lock会返回MutexGuard，LockResult包裹的，MutexGuard实现了Deref，也实现了Drop，可以在出作用域时自动释放锁
        let mut num = m.lock().unwrap();    // lock可以阻塞线程直到lock被释放
        *num = 6;
    }

    println!("m = {:?}", m);

    // 10个线程使用Mutex
    // let counter = Rc::new(Mutex::new(0));   // 这种方法不行，要用Arc，atomic reference count，线程才安全
    let counter = Arc::new(Mutex::new(0));  // 默认不在Rc实现Arc的功能是因为有性能损耗
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Rc::clone(&counter);  // 用Rc来创建多所有权，但是会报错，他不是线程安全的，send方法没有实现Rc<Mutex<T>>
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}