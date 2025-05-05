use std::str::FromStr;
use std::num::IntErrorKind;

fn parse_integer_from_string(input: &str) -> Result<i32, String> {
    // https://doc.rust-lang.org/stable/std/primitive.i32.html#impl-FromStr-for-i32
    match i32::from_str(input) {
        Ok(num) => Ok(num),
        // https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
        // #[non_exhaustive]
        Err(e) => match e.kind() {
            IntErrorKind::Empty => Err("String was empty".to_string()),
            IntErrorKind::InvalidDigit => Err("Invalid digit found".to_string()),
            IntErrorKind::PosOverflow => Err("Positive over flow".to_string()),
            IntErrorKind::NegOverflow => Err("Negative over flow".to_string()),
            IntErrorKind::Zero => Err("Value was Zero".to_string()),
            _ => Err("Unknown error".to_string())
        }
    }
}

fn main() {
    // let result = parse_integer_from_string("999");
    // let result = parse_integer_from_string("00000");
    let result = parse_integer_from_string("-99999999999999999999999999999999999999999999");
    // let result = parse_integer_from_string("abc");
    match result {
        Ok(num) => println!("Parsed number: {}", num),
        // https://doc.rust-lang.org/stable/std/num/struct.ParseIntError.html#method.kind
        // https://doc.rust-lang.org/stable/std/num/enum.IntErrorKind.html
        Err(e) => println!("Error parsing number: {}", e),
    }
}