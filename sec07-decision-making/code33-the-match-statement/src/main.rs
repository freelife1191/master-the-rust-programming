fn main() {
    let x: u8 = 5;

    match x {
        // 1 => println!("one"),
        // 2 => println!("two"),
        // _ => println!("Something else"), // other 처리를 하지 않으면 Error
        1 => {
            println!("one");
            println!("1");
        }
        2 => {
            println!("two")
        }
        _ => { // other 처리를 하지 않으면 Error
            println!("Something else")
        }
    }
}
