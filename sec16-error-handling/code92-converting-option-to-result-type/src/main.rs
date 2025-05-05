fn add_strings(s1: &str, s2: &str) -> Option<String> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        None
    } else {
        Some(format!("{} {}", s1, s2))
    }
}
fn main() -> Result<(), String>{
    /*
    Converting Option type to Return type
     */
    let s1 = String::new();
    let s2 = String::from("world!");

    // https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
    let res = add_strings(&s1, &s2).ok_or("Strings cannot be empty")?;
    println!("{}", res);
    Ok(())
}
