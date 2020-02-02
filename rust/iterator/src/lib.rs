#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // iter使用的是不易变引用，也可以用into_iter夺取所有权，iter_mut使用易变引用
    let mut v1_iter = v1.iter();    // next调用会消耗迭代器，所以v1_iter要定义为mut

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // 迭代器还有好多方法
    let total: i32 = v1_iter.sum(); // sum调用后就不法再使用v1_iter了，夺去了所有权

    assert_eq!(total, 6);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()   // 这里迭代器拿走了所有权
        .filter(|s| s.size == shoe_size)    // 过滤执行为true的会留下来，保留在迭代器中
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 实现自定义的Iterator trait，从1迭代到5
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));    // 调用自定义的迭代器
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))   // 可以忽略迭代器，从2开始
                                 .map(|(a, b)| a * b)   // 1*2 2*3 3*4 4*5 5*None，None会被丢弃
                                //  .filter(|x| x % 3 == 0)
                                 .sum();
    assert_eq!(18, sum);
}