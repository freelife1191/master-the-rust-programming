
### Struct std::fmt::Formatter

https://doc.rust-lang.org/stable/std/fmt/struct.Formatter.html

- `Format` 구조는 서식 옵션을 지정하고, 너비, 정밀도, 정렬을 설정하고, 기타 서식 작업을 수행할 수 있는 메서드와 사용 방법을 제공한다.
- It serves as a destination for writing formatted data
  - `Formatter` 객체는 일반적으로 호출 시 서식 지정 매크로(예: `println!`, `write!`, `format!`)에 의해 제공되며 명시적으로 직접 생성하지는 않는다
  - 출력 스트림(문자열, 파일, 표준 출력 등)을 캡슐화하고 형식화된 데이터를 쓰기 위한 방법을 제공한다

### write!()

```rust
let mut buffer = String::new();
use std::env::args;

write!(buffer, "Your message {} {}", arg1, args, ...)
```

- `write!` 매크로는 지정된 출력 스트림에 형식화된 데이터를 쓰는 데 사용된다
- `buffer`는 포맷된 출력이 기록될 대상(출력 스트림)입니다. `String`, `Vec<u8>` 또는 파일 스트림과 같이 `std::fmt::Write` 특성을 구현하는 모든 유형일 수 있다

```rust
fn main() {
  let my_dog = Dog {
    weight: 10,
    age: 2,
    name: String::from("Nacho"),
  };
  
  println!("{}", my_dog);
  println!("{:15}", my_dog);
}
```

`fmt::Display` 특성을 구현하여 `println!` 또는 `format!`과 같은 형식 지정 매크로를 사용할 때 유형이 인쇄되거나 표시되는 방법을 사용자 정의하고 유형이 문자열로 형식 지정되는 방법을 정의한다

`#` 채우기로 가운데 정렬

- `{}`: 기본 정렬. 지정된 너비 내에서 인수를 왼쪽 정렬
- `{:>width}`: 지정된 너비 내에서 인수를 오른쪽 정렬
- `{:width}`: 지정된 너비 내에서 인수를 왼쪽 정렬
- `{:^width}`: 지정된 너비 내에서 인수를 가운데 정렬
- `{:=^width}`: 채우기 문자로 '='를 사용하여 지정된 너비 내에서 인수를 가운데 정렬