use std::io;
use rand::Rng;  // Rng为trait，定义了随机数生成器
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // thread_rng获取当前线程和随机数种子，gen_range是Rng引入的方法，生成1-100随机数
    // 不知道引入的库如何使用，可以通过cargo doc --open查看文档
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);


    // 无限循环
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        // stdin返回io::Stdin终端输入对象
        // read_line获取用户输入
        // 用户输入会作为字符串存储在guess，可修改字符串值，所以是mut
        // &表示引用，传进来参数还是会指向外面的变量，用同一块存储空间
        // read_line返回io::Result枚举类型，有Ok和Err两个值，Err时会包含错误信息，使用Result的expect方法来处理错误
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // shadow前面的guess变量，不需要重新建变量了
        // trim去掉前后空格，parse将字符串转为数字，指定类型为u32
        // parse返回的也是Result类型，可以通过match语句来匹配
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // 转换成功则返回num
            Err(_) => continue, // 其他情况则会继续下一轮循环
        };

        // {}是占位符，用后面的参数替换
        println!("You guessed: {}", guess);

        // Ordering也是枚举类型
        // match类似于switch语句，匹配上了就停止，不用加break
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // 跳出循环
            }
        }
    }
    
}
