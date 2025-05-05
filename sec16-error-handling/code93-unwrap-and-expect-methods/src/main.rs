fn add_strings(s1: &str, s2: &str) -> Option<String> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        None
    } else {
        Some(format!("{} {}", s1, s2))
    }
}
fn main() {
    /*
    Unwrap() and expect()
     */
    let s1 = String::from("Hello");
    let s2 = String::from("world!");

    // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
    // let res = add_strings(&s1, &s2).unwrap();
    // https://doc.rust-lang.org/std/option/enum.Option.html#method.expect
    let res = add_strings(&s1, &s2).expect("Strings cannot be empty");
    println!("{}", res);
}
