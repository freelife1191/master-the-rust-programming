# Decision marking in Rust

- `if`...`else`
- `if`...`else if`...`else`
- `if let`
- `match`

Rust에는 두 가지 주요 의사 결정 문이 있습니다:

1. `if`/`else statement`: 코드의 조건부 실행에 사용됩니다
2. `match statement`: 패턴 일치에 사용되며 종종 여러 if 문 대신 사용됩니다

## The `if and else` statement

```
if expression {
    // code to run if the expression is true
} else {
    // code to run if the expression is false
}
```

`if` 문의 `expression`은 `true` 또는 `false`의 부울 값으로 평가되어야 합니다

```rust
fn main() {
    // if and else statement in Rust
    let age = 10;
    
    if age < 10 {
        println!("you cannot vote");
    } else {
        println!("you can vote");
    }
    
}
```

## Expression vs Statement

표현식은 값으로 평가되지만 문은 그렇지 않습니다
Rust에서는 `if`와 `match`를 모두 값을 반환하는 방식으로 사용하여 표현식으로 만들 수 있습니다

> Rust에서는 `if`와 `match`를 표현식으로 사용하여 값을 반환할 수 있습니다

https://doc.rust-lang.org/stable/std/primitive.unit.html

## `if else` used as an expression

```rust
fn main() {
    let age = 10;

    let message = if age < 18 {
        "You cannot vote" // Note: no ; here
    } else {
        "You can vote" // Note: no ; here
    }; // Note: ; here

    println!("{}", message);
}
```

- "투표할 수 없습니다"라는 식이 반환되면 `if` 표현식의 첫 번째 분기가 실행됩니다
- 문자열이 false인 경우 `if` 표현식의 두 번째 분기가 실행되고 "You can vote" 문자열이 반환됩니다
- Rust에서 `if`와 `else` 블록 표현식은 동일한 유형의 값을 생성해야 한다는 점에 유의하는 것이 중요합니다
- `else` 블록이 없는 `if` 블록 표현식은 `()`로 평가됩니다

Rust에서 코드 블록의 최종 값으로 사용되는 표현식에는 세미콜론이 필요하지 않으며 블록의 값은 마지막 표현식의 값입니다