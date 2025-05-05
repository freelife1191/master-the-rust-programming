### while let

Rust 의 `while let` 루프는 패턴이 값과 일치하는 한 코드 블록의 반복 실행을 허용합니다

```
while let pattern = value {
    ...
}
```

### When to use while let loop?

- 패턴이 계속 일치하는 한 반복하고 싶을 때 이것을 사용하십시오

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut iterator = numbers.iter();

    while let Some(number) = iterator.next() {
        if number % 2 == 0 {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }
    }
}
```

`while let` 루프는 패턴 일치를 사용하여 옵션이나 결과를 구조 해제하는 루프를 표현하는 보다 간결한 방법입니다  
반복 횟수를 알 수 없고 입력에 따라 달라질 때 유용합니다