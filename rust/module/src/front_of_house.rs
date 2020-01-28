pub mod hosting {
    pub fn add_to_waitlist() {}
}

// 可以不给出定义，再次进行拆分，把定义放到src/front_of_house/hosting.rs里
// rust会去查找和mod同名的文件
pub mod hosting