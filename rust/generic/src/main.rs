/**
 * 泛型用来解决代码重复问题
 * trait用来限定泛型的类型
 */

fn main() {
    // 找列表的最大值功能
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // 另一组找最大值，与上面功能重复，通过提取函数来抽象出来相同功能
    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let result = largest(&number_list); // 通过多次调用函数来实现，可以避免代码修改可能带来的漏改错误

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // 泛型方法的调用
    let result = largest(&number_list);
    let result = largest(&char_list);

    // 泛型结构体
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // 用第一种定义，则xy是同类型的，不同类型会报类型不匹配错误，那么用两个泛型参数的定义就可以
    let wont_work = Point { x: 5, y: 4.0 };

    // 调用x泛型方法
    println!("p.x = {}", p.x());

}

// 入参表示任意list的切片
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// i32类型的找最大值
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// char类型的找最大值
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 定义一个泛型函数，他会用到T类型，入参是T类型的切片，返回值也是T类型的
// 这个函数编译会报错，因为不是所有的类型都可以比大小，报错提示没有实现std::cmp::PartialOrd trait
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 结构体定义也可以用泛型参数
struct Point<T> {
    x: T,
    y: T,
}

// 多个泛型参数，可对应不同类型。但是泛型参数不是越多越好，过多之后说明该拆分或者简化
struct Point<T, U> {
    x: T,
    y: U,
}

// 枚举也可以用泛型
enum Option<T> {
    Some(T),
    None,
}

// 也可以是多个参数的泛型。发现多个结构体或者枚举只是类型不同时，就可以用泛型来消除代码重复
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 在impl中声明T，说明在方法中可以使用T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 也可以实现仅某种类型的方法，不需要在impl后加泛型参数，只有Point<f32>类型的对象才有该方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// impl的泛型参数是结构体的，函数的泛型参数是另外一套，要区分开他们
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/**
 * let integer = Some(5);
 * let float = Some(5.0);
 * 对泛型编译时，编译器使用单体化编译，会搜索程序中会用到的所有参数类型，生成多个针对不同类型的定义
 * 实际编译后像下面一样
 * let integer = Option_i32::Some(5);
 * let float = Option_f64::Some(5.0);
 * 所以使用泛型不会有任何性能损失
 */
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}