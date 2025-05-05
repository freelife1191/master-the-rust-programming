// fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
fn add_strings(s1: &str, s2: &str) -> Result<String, i32> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        // Err("Strings cannot be empty".to_string())
        Err(10)
    } else {
        Ok(format!("{} {}", s1, s2))
    }
}
fn main() {
    let s1 = String::from("");
    let s2 = String::from("world!");

    // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err
    // let error = add_strings(&s1, &s2).unwrap_err();
    // println!("{}", error);
    // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or
    let res = add_strings(&s1, &s2).unwrap_or("Strings cannot be empty".to_string());
    println!("{}", res);
}
