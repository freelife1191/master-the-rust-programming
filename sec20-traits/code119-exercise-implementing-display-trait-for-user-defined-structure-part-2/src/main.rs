use std::fmt;
use std::ptr::write;

struct Dog {
    weight: u8,
    age: u8,
    name: String,
}

impl fmt::Display for Dog {
    // https://doc.rust-lang.org/stable/std/fmt/type.Result.html
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#^17}", "Dog details").unwrap();
        Ok(())
    }
}

fn main() {
    let my_dog = Dog {
        weight: 2,
        age: 2,
        name: "Bow Wow".to_string(),
    };

    println!("{}", my_dog);
}