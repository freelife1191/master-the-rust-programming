# Function definitions

-`fn` 키워드 뒤에 함수 이름과 매개변수 목록을 사용하여 함수를 정의할 수 있습니다. 함수 본문은 중괄호로 묶입니다

```rust
fn main() {
    greet("Ram");
}

// &str 유형(문자열 슬라이스)의 단일 매개변수 이름을 사용합니다
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```


```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let greeting = greet("Ram");
    println!("{}", greeting);
}
```


## Legal function names

잘못된 함수 이름

- `1function` (숫자로 시작할 수 없습니다)
- `add-two-numbers` (하이픈을 사용할 수 없습니다)
- `fn` (예약어를 함수 이름으로 사용할 수 없습니다)
- `my function` (공백을 사용할 수 없습니다)
- `print_hello_world!` (밑줄을 제외한 특수문자는 사용할 수 없습니다.)