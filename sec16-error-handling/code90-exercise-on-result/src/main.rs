enum AddStringError {
    EmptyString,
    LengthMismatch,
}
impl AddStringError {
    fn description(&self) -> &str {
        match self {
            Self::EmptyString => "Empty string detected",
            Self::LengthMismatch => "Length did not match",
        }
    }
}
fn add_strings(s1: &str, s2: &str) -> Result<String, AddStringError> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        return Err(AddStringError::EmptyString);
    }
    if s1.len() != s2.len() {
        return Err(AddStringError::LengthMismatch);
    }
    Ok(format!("{} {}", s1, s2))
}
fn main() {
    /*
     Exercise
     */
    // let c = add_strings("", "");
    let c = add_strings("Hello", "World");
    match c {
        Ok(str) => println!("{}", str),
        Err(e) => println!("Error reason: {}", e.description()),
    }
}
