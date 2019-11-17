fn main() {
    // 这种方式没有将两个变量结合，他们之间是有关系的
    // let width1 = 30;
    // let height1 = 50;
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // 元组没有名字
    // let rect1 = (30,50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // 结构体表达了关系也有字段的描述
    let rect1 = Rectangle { width: 30, height: 50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(&rect1)
        rect1.area()    // 通过.符号调用，rust没有像Cpp一样的->，rust会自动引用或者解引用
    );

    // println!("rect1 is {}", rect1);  // 这样是不行的，因为结构体打印格式没有固定下来
    println!("rect1 is {:?}", rect1);   // 这种方式是以调试格式打印
    println!("rect1 is {:#?}", rect1);  // 以json格式显示

//////////////////////////////////////////////////

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
//////////////////////////////////////////////////////////

    let sq = Rectangle::square(3);
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 调试打印需要值此声明注解
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用impl定义函数，可以将函数分开写在多个impl中
impl Rectangle {
    // 方法也用fn定义，第一个参数往往是self，表示调用他的实例，使用&来borrow，如果想改变值，可以定义为&mut self
    // 一般不使用self，原实例调用方法后就会被move
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 没有self的调用时用::，叫关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}