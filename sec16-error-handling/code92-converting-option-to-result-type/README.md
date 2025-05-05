## Converting Option type to Return type

`ok_or` 메소드를 사용하여 `Option`을 예상 `Result` 유형과 호환되는 오류 값이 있는 `Result`로 변환할 수 있습니다.


```rust
fn add_strings(s1: &str, s2: &str) -> Option<String> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        None
    } else {
        Some(format!("{} {}", s1, s2))
    }
}
fn main() -> Result<(), String>{
    let s1 = String::new();
    let s2 = String::from("world!");

    // https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
    // 이 인수는 add_strings가 None 변형을 반환할 때만 사용됩니다
    let res = add_strings(&s1, &s2).ok_or("Both strings must be non-empty")?;
    println!("{}", res);
    Ok(())
}
```