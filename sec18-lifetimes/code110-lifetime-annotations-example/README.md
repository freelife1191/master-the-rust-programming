
## Explicit Lifetime annotation

```rust
fn extend_string<'a>(original: &'a mut String, data: &'a str) -> &'a str {
    ...
}
```

## Lifetime annotation syntax

다음은 수명 주석의 몇 가지 예

- `'a` : a 라는 이름의 수명 주석
- `'b` : b 라는 이름의 수명 주석
- `'xyz` : xyz 라는 이름의 수명 주석
- `'_` : 무명 수명 주석