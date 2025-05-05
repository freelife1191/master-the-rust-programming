# Exercise

형식 지정자 `{}`를 사용하여 구조를 인쇄하는 `Display` trait 을 구현한다

```rust
struct Dog {
    weight: u8,
    age: u8,
    name: String,
}

fn main() {
    let my_dog = Dog {
        weight: 2,
        age: 2,
        name: "Bow Wow".to_string(),
    };

    println!("{}", my_dog);
}
```

## Implementing the Display trait

https://doc.rust-lang.org/stable/std/fmt/trait.Display.html

- Display trait 은 자동으로 파생될 수 없다. 형식화된 방식으로 표시하려는 각 구조체 또는 열거형에 대해 수동으로 구현해야 한다
- `std::fmt::Display` trait 은 문자열로 표시할 객체의 형식을 지정하는 동작을 구체적으로 정의한다. 객체가 인쇄되거나 문자열로 변환되는 방법을 사용자 정의하려는 경우 일반적으로 사용된다
- 유형에 대해 `std::fmt::Display` trait 을 구현하면 `println!`, `format!()`, `write!( )` 또는 명시적으로 문자열로 변환하는 경우

### Module std::fmt

- `std::fmt` 모듈은 유형의 형식 지정 동작을 사용자 정의하기 위해 구현할 수 있는 여러 특성을 정의한다. 가장 일반적으로 사용되는 특성은 `fmt::Display`로, 유형이 문자열로 표시될 때 형식이 어떻게 지정되는지 정의하는 데 사용된다




### Trait std::fmt Display

```rust
use std::io::Error;

pub trait Display {
    // Required method
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

https://doc.rust-lang.org/stable/std/fmt/index.html#types

- `fmt` 메소드는 `fmt::Display` 특성의 필수 메소드이다. `fmt` 메소드 내에서 원하는 형식에 따라 self 객체의 형식을 지정하는 코드를 작성한다
- `fmt::Formatter` 매개변수는 형식 지정 옵션을 적용하고 형식화된 출력을 작성하는 방법을 제공한다
  - https://doc.rust-lang.org/stable/std/fmt/struct.Formatter.html
- `Ok(())`: 성공적인 포맷 작업을 나타낸다
- `Err(std::fmt::Error)`: 포맷 과정에서 발생한 오류를 나타낸다
  - https://doc.rust-lang.org/stable/std/fmt/struct.Error.html
  - https://doc.rust-lang.org/stable/std/fmt/type.Result.html


`std::fmt`를 가져와야 한다  
이 가져오기가 없으면 컴파일러는 사용자가 어떤 Display trait 을 참조하고 있는지 알 수 없다  
Display trait 을 정의하는 모듈이 많이 있을 수 있기 때문이다

```rust
// `fmt` module from the Rust standard library
use std::fmt;

impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dog: weight: {}, age: {}, name: {}", self.weight, self.age, self.name)
    }
}
```

Dog 유형에 대한 `fmt::Display` trait 구현