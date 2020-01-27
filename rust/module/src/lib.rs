mod front_of_house {    // 模块用mod关键字定义
    pub mod hosting {   // 可以嵌套。使用pub关键字设置公有，模块设置pub并不会作用在他内部的函数等等
        pub fn add_to_waitlist() {} // 模块里可以定义函数，也可以是其他类型，结构体、枚举、常量或者trait
        // src/main.rs和src/lib.rs构成一个crate的模块，这是一个隐含模块
        // crate
        // └── front_of_house
        //     ├── hosting
        //     │   ├── add_to_waitlist
        //     │   └── seat_at_table
        //     └── serving
        //         ├── take_order
        //         ├── serve_order
        //         └── take_payment
       
        fn seat_at_table() {}
    }

    mod serving {   // mod会添加一层私有边界，他默认是私有模块。子模块可以访问父模块，但是父模块无权访问私有子模块，子模块对外隐藏了细节
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// path可以有两种形式：
// 1.绝对路径，从某个crate名开始或者从crate模块自身开始
// 2.相对路径，从从当前模块，用self、super或当前模块的标识符
// 路径之间用::分隔
pub fn eat_at_restaurant() {    // 和front_of_house是兄弟关系，所以可以访问
    // 绝对path，类似文件目录结构，crate相对于/，::相当于路径间的/
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对path，建议使用绝对路径
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();   // super相当于..，super后到达back_of_house模块的父级，也就是crate模块，可以访问同级的serve_order
    }

    fn cook_order() {}
}


mod back_of_house {
    pub struct Breakfast {  // pub用在结构体时，不会对其字段起作用，字段的访问权限需要另外设置
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {   // 必须有公有方法来设置私有字段seasonal_fruit，才能使用
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 私有字段不能访问，也就不能赋值
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house {
    pub enum Appetizer {    // 枚举类型则只需在enum前加pub就说明变体都是pub的
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}