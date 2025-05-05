fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        return Err(String::from("Empty string detected!"));
    }
    let result = format!("{} {}", s1, s2);
    Ok(result)
}
fn main() {
    // let s1 = "";
    // let s2 = "";
    let s1 = "Hello";
    let s2 = "World";

    match add_strings(s1, s2) {
        Ok(concatenated) => println!("{}", concatenated),
        Err(error) => println!("{}", error),
    }
}
