
## Associated type of a trait

```rust
trait Animal {
    fn make_sount(&self);
    type Weight;
    fn set_weight(&mut self, weight: Self::Weight);
    fn get_weight(&self) -> Self::Weight;
    fn set_age(&mut self, age: u8);
    fn get_age(&self) -> u8;
}
```

- 관련 유형 `Weight`는 구현 구조체 또는 클래스가 정의할 특정 구체적인 유형에 대한 자리 표시자 역할을 한다. 이는 특성을 보다 일반적이고 유연하게 만드는 방법으로, 각 구현 유형이 Weight에 대해 고유한 구체적인 유형을 지정할 수 있도록 한다
- `Weight`가 나타내는 특정 유형은 관련 유형에 대한 자체 구현을 제공할 때 구현 구조체 또는 클래스에 의해 결정된다
- 관련 유형 이름에 PascalCase(UpperCamelCase라고도 함)를 사용하는 것은 Rust의 일반적인 규칙이다

## Type aliasing

```rust
// Creating a type alias for u8
type Byte = u8;

fn main() {
    let a: Byte = 255;
    println!("The value of a is: {}", a);
}
```

```rust
fn main() {
    let origin: (i32, i32) = (0, 0);
    println!("The origin is at: ({}, {})", origin.0, origin.1);
}

// Creating a type alias for a tuple
type Point = (i32, i32);

fn main() {
    let origin: Point = (0, 0);
    println!("The origin is at: ({}, {})", origin.0, origin.1);
}
```