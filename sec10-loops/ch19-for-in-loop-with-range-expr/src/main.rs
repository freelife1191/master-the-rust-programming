/* Rust의 반복자인 범위 표현식이 포함된 for..in 루프의 예 */
fn main() {
    'outer: for i in 0.. {
        println!("{}", i);
        if i == 10 {
            break 'outer;
        }
    }


    // Error attempt to add with overflow 발생
    /*
    let start = i32::MAX - 5;
    println!("i32::MAX is {}", i32::MAX);
    for i in start.. {
        println!("{}", i);
    }
    */

    // a range is an iterator in rust
    for i in -5..=5 {
        println!("{:2}", i);
    }
}
