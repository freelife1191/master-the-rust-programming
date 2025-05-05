fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// rust-lang.org
// doc.rust-lang.org/std/primitive.i32.html

fn main() {
    let value: u16 = 0;
    let sum = 10 + value;
    println!("sum is {}", sum);

    // Type inference
    let num1 = 5;
    let num2 = 10;

    println!("num1 is of type: {}", type_of(&num1));
    println!("num2 is of type: {}", type_of(&num2));
    let sum: u8 = num1 + num2;
    println!("sum is of type: {}", type_of(&sum));
    println!("sum is {}", sum);

    // Mutable and Immutable variables
    let mut num = 10;
    println!("num is {}", num);
    num = 20; // This will give an error
    println!("num is {}", num);

    // Rust enforces strict type checking
    let num1 = 20;
    let num2 = 4.5;
    let mul = num1 * (num2 as i32);
    println!("mul is {}", mul);

    let msg1 = "Hello".to_string();
    let msg2 = "Hello".to_string();
    // let another_msg = msg1 * msg2; // This will give an error

    // Line check attributes
    #[allow(overflowing_literals)]
    let num1: i32 = 2147483648;
    println!("num1 is {}", num1); // This will print -2147483648
}
