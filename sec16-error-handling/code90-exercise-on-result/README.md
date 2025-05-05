## Exercise

`Result<String, AddStringError>`를 반환하도록 `add_strings` 함수를 리팩토링하세요
오류에 대한 설명을 반환하려면 **AddStringError** 열거형에 `description()` 메서드를 구현하세요

```rust
enum AddStringError {
    EmptyString,
    LengthMismatch,
}
```

이 예는 다음을 수행하는 `add_strings` 함수를 보여줍니다  
두 개의 문자열 참조를 인수로 사용하고 결과를 반환합니다  
연결된 문자열이 포함되어 있거나 오류가 있는 경우 문자열 중 하나가 비어 있으면 메시지가 표시됩니다

```rust
fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
    // https://doc.rust-lang.org/std/primitive.str.html#method.is_empty
    if s1.is_empty() || s2.is_empty() {
        return Err(String::from("Empty string detected!"));
    }
    let result = format!("{} {}", s1, s2);
    Ok(result)
}
fn main() {
    let s1 = "";
    let s2 = "";

    match add_strings(s1, s2) {
        Ok(concatenated) => println!("{}", concatenated),
        Err(error) => println!("{}", error),
    }
}
```