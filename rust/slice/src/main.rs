fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("the first word is: {}", word);
    s.clear();
    // println!("the first word is: {}", word);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello world");
    let hello = &s[0..=4];
    let world = &s[6..=10];

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    let s = "Hello, world!";

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}