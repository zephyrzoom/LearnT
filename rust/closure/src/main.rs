/**
 * 闭包是匿名函数，可以赋值给变量，也可以作为函数的参数
 */
use std::thread;
use std::time::Duration;
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// }

// fn generate_workout(intensity: u32, random_number: u32) {
//     let expensive_result = simulated_expensive_calculation(intensity); // 提取函数

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_result);
//         println!("Next, do {} situps!", expensive_result);
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!"); // 这里执行时也计算了expensive_result，但是其实是用不到的
//         } else {
//             println!("Today, run for {} minutes!", expensive_result);
//         }
//     }
// }

// 使用闭包
// fn generate_workout(intensity: u32, random_number: u32) {
//     // 闭包用|包裹参数，可以有多个参数，函数返回值就是闭包的值。这里只是定义了函数，并没有调用
//     // 闭包函数不是暴露给外界用的，没有名字，所以参数类型的注解也是没有的，不需要
//     let expensive_closure = |num| {
//         // 使用闭包就不必一开始就计算函数了，即使在不需要的时候。现在只是把函数存起来
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };

//     // let expensive_closure = |num: u32| -> u32 {
//     //     // 使用闭包就不必一开始就计算函数了，即使在不需要的时候。现在只是把函数存起来
//     //     println!("calculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num
//     // };

//     // 函数与闭包的对比，他们都是一样的
//     // fn add_one_v1(x: u32) -> u32 { x + 1 }
//     // let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     // let add_one_v3 = |x| { x + 1 };
//     // let add_one_v4 = |x| x + 1; // 只有一个表达式，所以不用括号

//     // let example_closure = |x| x;
//     // let s = example_closure(String::from("hello"));  // 第一次执行编译器已进行了类型推断，入参和返回都是String，会固定下来
//     // let n = example_closure(5);  // 会报错，类型不匹配


//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_closure(intensity) // 和普通函数调用方式一样
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_closure(intensity) // 这里还是调用了两次，应该想办法缩减为一次
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }
// }

struct Cacher<T>
    where T: Fn(u32) -> u32 // Fn是trait，这里是trait限定
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // 访问value时，会执行闭包，并将技术结果缓存在结构体对象中，是一个Some变体
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,   // 有值则直接返回
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

// 存在两个问题
// 1.每次返回都是一样的值
// 2.参数类型被限制了
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);    // 返回的还是第一次的1，以后每次调用都是一样的

    assert_eq!(v2, 2);
}

/**
 * 闭包使用环境作用域的变量有三种方式：获取所有权、易变借用和不易变借用
 * 对应了三个trait：FnOnce、FnMut和Fn
 */
fn main() {
    let x = 4;

    // 这里x属于不易变借用，其实是使用了Fn
    let equal_to_x = |z| z == x;    // 闭包还有个函数不具备的功能：他可以访问闭包定义处作用域的值，可以使用x的值

    let y = 4;

    assert!(equal_to_x(y));
}

fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }    // 函数中，这样是不行的

    let y = 4;

    assert!(equal_to_x(y));
}

fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;   // 使用move关键字就是用了FnMut，闭包会获得x的所有权

    println!("can't use x here: {:?}", x);  // 这里main函数没有了x的所有权，无法访问

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}