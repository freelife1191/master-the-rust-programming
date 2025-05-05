fn main() {
    /*
    let start = i32::MAX - 5;
    println!("i32::MAX is {}", i32::MAX);
    for i in start.. {
        println!("{}", i);
    }
    */
    /*
    for i in (0..100).step_by(2) {
        println!("{}", i);
    }
    */
    // https://doc.rust-lang.org/stable/std/primitive.array.html
    // https://doc.rust-lang.org/stable/std/primitive.array.html#impl-IntoIterator-for-%26%5BT;+N%5D
    /*
    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }
    */
    // https://doc.rust-lang.org/stable/std/primitive.str.html#method.chars
    let msg = "Hello, world!";
    let target_char = 'l';
    let mut count = 0;
    for c in msg.chars() {
        if c == target_char {
            count += 1;
        }
    }
    println!("{}", count);
}