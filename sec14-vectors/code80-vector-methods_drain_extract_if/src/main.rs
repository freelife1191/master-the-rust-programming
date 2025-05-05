/*
fn filter_number(e: &mut i32) -> bool {
    *e > 5
}
*/


#![feature(extract_if)] // Channel feature is required to use extract_if
fn main() {
    /*
    drain()
    */
    // let mut v = vec![1, 2, 3, 4, 5];

    /*
    let mut v_new = vec![];

    let _ = v.drain(1..3); // This will drain values 2 and 3

    println!("{:?}", v);
    println!("{:?}", v_new);
    */
    /*
    for e in v.drain(..) {
        println!("{}", e);
    }
    println!("{:?}", v);
    */

    // https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.extract_if
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];
    // let c1: Vec<_> = numbers.extract_if(filter_number).collect();
    let c1: Vec<_> = numbers.extract_if(|e| *e > 5).collect();
    println!("{:?}", numbers);
    println!("{:?}", c1);
}