// use cargo::kinds::PrimaryColor;
// use cargo::utils::mix;
use cargo::mix; // 重新导出模块后，方便使用
use cargo::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}