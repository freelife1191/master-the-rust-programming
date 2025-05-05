# 'Option' and 'Result' Shorthand

- Rust는 'Option' 및 'Result' 유형에 대한 내장된 단축어를 제공합니다
- `Option::Some(value)` 대신 `Some(value)`를, `Result::Ok(value)` 대신 `Ok(value)`를 사용할 수 있습니다
- 이 약칭은 이러한 유형을 사용하는 코드를 훨씬 더 깔끔하고 읽기 쉽게 만듭니다
- `Option<T>` 및 `Result<T, E>`는 Rust 서문에 포함되어 있습니다. 즉, 명시적으로 범위에 포함할 필요 없이 모든 Rust 프로그램에서 자동으로 사용할 수 있다는 의미입니다
- 서곡은 Rust가 모든 Rust 프로그램에 자동으로 가져오는 것들의 목록입니다