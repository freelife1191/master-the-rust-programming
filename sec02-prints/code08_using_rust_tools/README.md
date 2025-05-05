# 깔끔하고 효율적이며 관용적인 Rust 코드 작성

Recommended tools for Rust developers

## `rustfmt`

이 도구는 커뮤니티 스타일 지침에 따라 Rust 코드의 형식을 자동으로 지정합니다. 이는 프로젝트 전체의 코드가 일관된 스타일을 갖도록 보장하며 이는 특히 협업 환경에 유용합니다

- https://github.com/rust-lang/rustfmt


```rust
fn main() {
    let name = "John";
    let age = 30;

    let msg = format!(
        "My name is {user_name} and I am {user_age} years old",
        user_age = age,
        user_name = name
    );
    println!("{}", msg);
}
```


## `Clippy`

Rust용 정적 분석 도구로, 종종 linter라고 불리거나 Lint 검사를 제공합니다. Rust 컴파일러 자체가 반드시 포착할 수 없는 광범위한 문제를 찾기 위해 Rust 코드를 실행하지 않고 분석합니다

- https://github.com/rust-lang/rust-clippy


```rust
fn main() {
    let x = 100;
    let x = x * 1;
    println!("{}", x);
}
```

```rust
fn main() {
    let array = [1, 2, 3];
    // for i in 0..=3 { // error: the range `0..=3` does not have a defined end point
    for i in 0..=2 {
        println!("{}", array[i]);
    }
}
```

```rust
fn main() {
    let array = [1, 2, 3];
    
    for item in &array {
        println!("{}", item);
    }
}
```

## `cargo fix`

이 도구는 compiler(rustc)에서 제공하는 진단을 기반으로 Rust 코드에 수정 사항을 자동으로 적용하며, 최신 버전의 Rust 를 준수하도록 코드를 업그레이드할 때 특히 유용합니다


```rust
fn calc_circle_area(_radius: f32) -> f32 {
    3.14 * 4.0 * 4.0
}

fn main() {
    let pi = std::f32::consts::PI;
    let _area = pi * 4.0 * 4.0;
    // println!("{}", area);
    println!("{}", calc_circle_area(10.0));
}
```