# `format!()`

syntax:

`let output = format!("{}" ,value);`

- 여기서 `value`는 문자열에 삽입하려는 값입니다
- `{}`는 `value`에 대한 자리 표시자입니다
- 형식 문자열에 여러 개의 자리 표시자 `{}`를 사용하여 여러 값을 삽입할 수도 있습니다

```rust
fn main() {
    let name = "John";
    let age = 30;
    let message = format!("I am {} and I am {} years old", name, age);
    println!("{}", message);
}
```

이 예에서는 `format!` 매크로를 사용하여 `My name is {} and I am {} years old` 형식의 새 문자열을 생성하고 변수 `name` 및 `age` 값이 `placeholders`에 삽입됩니다. 결과 문자열은 `message` 변수에 저장됩니다



## Name placeholders

```rust
fn main() {
    let name = "John";
    let age = 30;
    // 이 자리 표시자는 'user_name'이라는 이름으로 식별됩니다
    // '\'는 가독성을 높이기 위해 긴 문자열 리터럴을 소스 코드의 여러 줄로 분할하는 데 도움이 됩니다
    let message = format!(
        "My name is {user_name} and I am {user_age} \
        years old and its very long message",
        user_age=age, user_name=name);
    println!("{}", message);
}
```

`\`는 소스 코드에서 하나의 긴 문자열을 여러 줄로 분할하는 데 사용됩니다. 반면 문자열은 처리될 때 여전히 한 줄로 처리되므로 코드를 가독성을 위해 여러 줄로 작성할 수 있습니다