fn main() {
    /*
    // let closure = |x: i32| -> i32 {
    let closure = |x| {
        // return x + 1;
        x + 1
    };
    */
    // let closure = |x| x + 1;
    // println!("{}", closure(10));

    let add = |a, b| a + b;

    let result1 = add(5, 7);
    println!("Result 1: {}", result1);

    // Error: mismatched types
    // let result2 = add(5, 7.0);
    // println!("Result 1: {}", result2);
}
