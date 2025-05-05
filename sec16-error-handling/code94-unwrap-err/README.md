## unwrap_err()

`unwrap_err()`은 `결과`가 `Err`이 아닌 경우 패닉 상태가 됩니다  
이는 `결과`가 `Ok`가 아닌 경우 당황하는 `unwrap()` 메서드와 대응됩니다


```rust
fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        return Err(String::from("Both strings must be non-empty"));
    }
    Ok(format!("{} {}", s1, s2))
}
fn main() {
    let s1 = String::new();
    let s2 = String::from("world!");
    
    let result = add_strings(&s1, &s2);
    if result.is_ok() {
        println!("Concatenated string: {}", result.unwrap());
    } else {
        // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err
        println!("{}", result.unwrap_err());
    }
}
```

## unwrap_or(fallback)

`unwrap_or(fallback)`은 `Some` 또는 `Ok` 변형의 포함된 값을 반환하거나 `None` 또는 `Err`인 경우 인수로 제공된 기본값을 반환하는 `Option` 및 `Result` 열거형에 대해 정의된 메서드입니다