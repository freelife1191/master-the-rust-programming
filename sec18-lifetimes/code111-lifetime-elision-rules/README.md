
## Lifetime elision rules

1. 기능 매개변수의 적절한 수명을 설정하기 위해 개별 수명 매개변수가 각 참조 매개변수에 할당된다.
2. 정확히 하나의 입력 수명 매개변수가 있는 경우 해당 수명은 모든 출력 수명 매개변수에 할당된다.
3. 입력 수명 매개변수가 여러 개 있지만 그 중 하나가 `&self` 또는 `&mut self`인 경우 `self`의 수명이 모든 출력 수명 매개변수에 할당된다.

- **Input lifetime**: 입력 수명은 함수에 인수로 전달되는 참조의 수명을 나타낸다
- **Output lifetime**: 출력 수명은 함수에서 반환된 참조의 수명을 나타낸다

### Lifetime elision rule #1

```rust
fn extend_string(original: &mut String, data: &str) -> &str {
    original.push_str(data);
    &original
}
```

↓ ↓ ↓ ↓ ↓ ↓ ↓

```rust
fn extend_string<'a, 'b>(original: &'a mut String, data: &'b str) -> &'a str {
    original.push_str(data);
    &original
}
```

### Lifetime elision rule #2

```rust
fn process_input(input: &str) -> (&str, &str) {
    (input, input)
}
```

↓ ↓ ↓ ↓ ↓ ↓ ↓

```rust
// No explicit lifetime annotations required
fn process_input<'a>(input: &'a str) -> (&'a str, &'a str) {
    (input, input)
}
```