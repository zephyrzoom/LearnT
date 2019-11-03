// 结构体可以比作有名字的元组，所以可以不按顺序，随机访问
fn main() {

    // 定义实例方法如下，结构体相当于模版，定义实例来赋值
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 如果是易变的，可以用点访问和赋值，易变一定是真的整个实例，不能是某个字段
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,   // 使用其他实例点字段赋值
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 缩写方法，其他字段使用user1的字段值
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    black.0 // 可以通过下标访问

    // 还可以创建无字段的结构体
}

// 定义方法就是这样
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 可以在方法中定义实例隐式返回
fn build_user(email: String, username: String) -> User {
    User {
        email,  // 使用和参数同名点变量，可以省略，相当于email:email
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 目前还无法使用引用类型定义结构体字段，需要配合生命周期，后面才讲
// struct User {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// 结构体元组，可以用来区分类型相同的元组
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);