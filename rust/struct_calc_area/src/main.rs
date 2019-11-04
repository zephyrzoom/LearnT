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
        rect1.area()
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

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}