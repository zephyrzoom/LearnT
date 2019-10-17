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
    // 定义同名变量会将之前的同名变量遮盖
    // 这种方式和mut是有区别的，必须每次都用let，每次声明后都是不易变值了
    let x = x + 1;
    let x = x * 2;

    println!("The value of x is: {}", x);
    // 遮盖的方式是可以改变变量类型的，但是mut就不能改变变量类型
    let spaces = "   ";
    let spaces = spaces.len();

    // 这里必须要设置类型
    let guess:isize = "42".parse().expect("Not a number!");

    /**
     * 四种标量：整型、浮点型、布尔和字符
     */
    // 整型：u表示无符号，i表示有符号
    // 大小必须符合，有8、16、32、64、128和arch（isize、usize），arch会根据系统架构设置位数
    // 如果整型溢出，debug模式会报溢出，但是在release模式会wrap数值。比如：u8范围是0-255，256就会变为0,257会变为1。也可以显式的调用Wrapping
    // let eight:u8 = 256;

    // 数值字面值可以设置在数字后面显示设置类型，也可以用_划分长数字
    // Decimal	        98_222
    // Hex	            0xff
    // Octal	        0o77
    // Binary	        0b1111_0000
    // Byte (u8 only)	b'A'
    // 浮点数有两种f32和f64，默认是f64
    let x = 2.0;

    // bool有两种值，可用于if表达式
    let t = true;
    let f:bool = false;

    // char为单引号，char为4个字节的Unicode编码
    let c = 'z';

    // 元组可以放任何类型，数量固定，声明后就不能修改了。还可以显式声明类型，tup作为一个单独元素
    let tup: (i32, f64, u8) = (5_000, 6.4, 1u8);

    // 可以通过解构来分解元组元素
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // 元组的元素可以通过.序号来访问
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 数组必须是相同类型元素，并且数组长度固定，不可修改，数组的空间分配在栈中。需要可变长度，用vector吧
    let a = [1, 2, 3, 4, 5];
    // 类型写在中括号中，类型和长度用分号分开
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 5个3的数组
    let a = [3; 5];
    let index = 10;
    // 用下标访问数组元素，当下标超出最大时，可以编译但是执行会报错panic，下标不合法是不会让你访问的，很安全
    let element = a[index];

    println!("The value of element is: {}", element);

    // 家国梦概率计算，结论：2张，3张
    // let one = 20.0 / (0.9 * (2.0/15.0) + 0.1);
    // let two = 100.0 / 2.0 / (0.45 * (2.0/15.0) + 0.55);
    let one = 20.0 / (0.1*(0.0/9.0));
    let two = 100.0 / 2.0 / (0.55*(0.0/9.0));
    let three = 500.0 / 3.0 / (0.53*(0.0/9.0) + 0.47);

    println!("one:{},two:{},three:{}", one, two, three);
}