fn apply(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg)
}

fn main() {
    let multiply = |x| x * x;
    // let y = 2;
    // let multiply = |x| x * y; // Error

    let result = apply(multiply, 4);
    println!("{}", result);

}
