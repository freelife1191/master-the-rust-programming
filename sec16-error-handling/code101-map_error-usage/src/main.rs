use std::io;
use std::str::FromStr;

/*
fn convert_err(e: std::num::ParseIntError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{}", e))
}
*/
fn parse_integer_from_string(input: &str) -> io::Result<i32> {
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#method.trim
    /*
    match i32::from_str(input.trim()) {
        Ok(num) => Ok(num),
        // https://doc.rust-lang.org/stable/std/io/struct.Error.html#method.new
        // https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#
        Err(e) => Err(io::Error::new(ErrorKind::InvalidData, format!("{}", e))),
    }
    */
    // https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.map_err
    // i32::from_str(input).map_err(convert_err)
    /*
    Closure
     */
    i32::from_str(input).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
}

fn main() -> io::Result<()> {
    // https://doc.rust-lang.org/stable/std/io/index.html#functions
    // https://doc.rust-lang.org/stable/std/io/trait.BufRead.html#method.read_line
    let mut user_input = String::new();
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