// 用fn定义函数，大括号里的为函数体
fn main() {
    // 宏调用是表达式
    println!("Hello, world!");
    // 函数名用小写字母组成下划线分割；函数可以定义在任何地方，没有位置要求；用括号调用，和其他语言一样
    // 函数的调用是表达式
    another_function(5, 15);


    // 函数体包含语句和表达式，语句没有返回值，表达式有结果值
    // 这是语句，main函数定义也是一个语句
    let y = 6;
    // 语句没有返回值，因此不能用它来赋值给其他变量，其他语言可以x=y=5，这里就不行
    // let x = (let y = 6);

    // 5+6是表达式，表达式可以是语句的一部分
    // {}也是表达式
    let x = {
        // 6是表达式，是这条语句的一部分
        let y = 6;
        // 表达式是没有分号的，加上分号就是语句
        y+1
    };
    println!("the value of x is {}", x);
    
    let x = five();
    println!("the value of x is {}", x);

    let x = plus_one(3);
    println!("the value of x is {}", x);

    let number = 12;
    // if后面跟条件
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // if条件必须是bool类型
    // if number {
    //     println!("number was three");
    // }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // if是表达式，所以可以用来赋值，每个分支必须有相同类型的返回值
    let number = if condition {
        "5"
    } else {
        "6"
    };
    println!("The value of number is: {}", number);

    // 循环，break退出循环
    loop {
        println!("loop again!");
        break
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // 可以通过break后的表达式返回loop跳出的结果
            break counter * 2;
        }
    };
    println!("loop break at: {}", result);
    assert_eq!(result, 20);

    let mut number = 3;

    // while循环，在while后跟执行条件
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while方式比for慢，每次要检查条件
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // for更安全，不会发生数组越界，如果修改了a也不会发生panic
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Range可以生成一定范围的数，rev用来反转序列
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// 有两个参数一个x和y，必须声明它们的类型
fn another_function(x: i32, y: i32) {
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}

// 函数通过->设置返回值，return返回表达式的值，默认会将最后一个表达式返回
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}