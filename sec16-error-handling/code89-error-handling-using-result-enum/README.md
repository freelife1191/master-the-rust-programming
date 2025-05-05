# Error handling in Rust

Rust는 프로그래머가 안전하고 효율적인 방식으로 오류를 처리할 수 있도록 여러 가지 메커니즘을 제공합니다

- Result Enum
  - The 'Result' enum
  - Error handling using 'Result' type
  - Panic (There are some scenarios where panicking my be more appropriate)
    - Out of memory errors
    - Invalid inputs or state
    - Unexpected errors
    - Debugging and testing
- Option Enum
- Panic macro
- unwrap() and expect() methods
- ? Operator

### The `Result` enum

`Result`는 성공하거나 실패할 수 있는 계산의 결과를 나타내는 Rust의 내장 유형입니다

```rust
enum Result<T, E> {
  Ok(T),
  Err(E),
}
```

- 결과 유형에는 `Ok`와 `Err`의 두 가지 가능한 변형이 있습니다
- `Ok` 변형은 성공적인 결과를 나타내며 작업이 성공할 경우 반환하는 값의 유형인 `T` 유형의 값을 포함합니다
- `Err` 변형은 실패한 결과를 나타내며 작업이 실패할 경우 반환하는 오류 유형인 `E` 유형의 값을 포함합니다


### Error handling using `Result` type

`Result` 유형은 주로 복구 가능한 오류를 처리하는 데 사용됩니다. 여기서 함수는 성공적인 값이나 오류 값을 반환할 수 있고 호출 코드는 의미 있는 방식으로 오류를 처리할 수 있습니다

- 예를 들어 프로그램이 파일에서 데이터를 읽어야 하는 경우 작업의 성공 여부를 나타내는 'Result' 유형을 반환할 수 있습니다. 그러면 호출 코드가 결과를 확인하고 오류를 적절하게 처리할 수 있습니다

> 복구 가능한 오류 처리에 사용됩니다


### Panic

- 'panic!'은 복구할 수 없는 오류가 발생하여 프로그램 실행이 중지되어야 함을 나타내는 데 사용할 수 있는 매크로입니다
- 패닉은 중요한 파일이나 리소스가 누락되거나 기능의 전제 조건이 위반되는 경우와 같이 복구할 수 없는 오류에 대해서만 드물게 사용해야 한다는 점에 유의하는 것이 중요합니다  
  대부분의 경우 'Result' 또는 'Option'을 사용하는 것이 더 나은 오류 처리를 가능하게 하므로 더 나은 선택입니다

> **_a 함수가 사용되는 경우_ **패닉!**, _본질적으로는_ **결정** _중단_ **프로그램 실행** _due to_ **복구할 수 없는 오류**.


### 당황하는 것이 더 적절할 수 있는 몇 가지 시나리오가 있습니다

1. **메모리 부족 오류**: 프로그램이 계속 실행하기에 충분한 메모리를 할당할 수 없는 경우 이 오류를 복구할 수 있는 방법이 없습니다
2. **잘못된 입력 또는 상태**: 프로그램에 잘못된 입력이 제공되거나 잘못된 상태에 들어가면 당황하는 것이 적절할 수 있습니다
3. **예기치 않은 오류**: 프로그램에서 처리 방법을 모르는 예기치 않은 오류가 발생하면 당황하는 것이 적절할 수 있습니다
4. **디버깅 및 테스트**: 개발 중에 패닉을 사용하여 코드의 오류와 버그를 식별하는 것이 유용할 수 있습니다  
   예를 들어, 테스트 사례가 실패하면 `panic!` 매크로를 사용하여 테스트가 실패했음을 알릴 수 있습니다
   `assert!` 매크로는 패닉을 사용하여 구현됩니다  
   `assert!` 매크로는 조건이 참인지 확인하는 데 사용되며, 그렇지 않으면 프로그램이 패닉 상태가 됩니다  
    이는 테스트 및 디버깅 중에 코드에 대한 가정이 올바른지 확인하는 데 유용할 수 있습니다


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