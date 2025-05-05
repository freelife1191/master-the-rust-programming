/**
Comparison operators in Rust
 **/
fn main() {
    /*
    let a = 10;
    let b = 20;
    let c = a == b; // type of c is bool
    let c = b > a;
    if a == b {} else if a > b {}
    */

    // Logical operatior in Rust
    let a = true; // bool
    let b = !a; // b is now false
    let c = a && b; // c is false
    println!("a:{a}, b:{b}, c:{c}")
}
