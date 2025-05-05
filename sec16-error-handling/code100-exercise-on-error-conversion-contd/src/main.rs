use std::io;
use std::io::{ErrorKind, Write};
use std::str::FromStr;

fn parse_integer_from_string(input: &str) -> io::Result<i32> {
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#method.trim
    match i32::from_str(input.trim()) {
        Ok(num) => Ok(num),
        // https://doc.rust-lang.org/stable/std/io/struct.Error.html#method.new
        // https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#
        Err(e) => Err(io::Error::new(ErrorKind::InvalidData, format!("{}", e))),
    }
}

fn main() -> io::Result<()> {
    // https://doc.rust-lang.org/stable/std/io/index.html#functions
    // https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.read_line
    let mut user_input = String::new();

    println!("Enter a number: ");
    // https://doc.rust-lang.org/stable/std/io/struct.Stdout.html#impl-Write-for-%26Stdout
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut user_input)?;

    let result = parse_integer_from_string(&user_input);
    match result {
        Ok(num) => println!("Parsed number: {}", num),
        // https://doc.rust-lang.org/stable/std/num/struct.ParseIntError.html#method.kind
        // https://doc.rust-lang.org/stable/std/num/enum.IntErrorKind.html
        Err(e) => println!("Error parsing number: {}", e),
    }
    Ok(())
}