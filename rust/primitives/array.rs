use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // [类型; 大小]
    let xs: [i32; 5] = [1,2,3,4,5];
    // 500个0
    let ys: [i32; 500] = [0; 500];
    println!("first element of array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // println!("{}", xs[5]);
}