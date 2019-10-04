fn main() {
    // 变量默认是不易变的，值和名绑定之后不能改变值，加上mut来指定他是可变的
    let mut x = 5;
    println!("The value of x is: {}", x);
    // 如果x是不易变的，则不能通过这句修改x
    x = 6;
    println!("The value of x is: {}", x);

    // 常量和不易变变量区别：
    // 1.常量用const定义，变量用let
    // 2.变量可以声明为mut，常量不行
    // 3.常量必须指定类型
    // 4.常量可以定义在任何位置
    // 5.常量值只能用常量表达式设置，不能通过运行时计算结果设置
    const MAX_POINTS: u32 = 100_000;

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();


    let guess:isize = "42".parse().expect("Not a number!");

    // let eight:u8 = 256;

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
