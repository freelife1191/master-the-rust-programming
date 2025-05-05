## Constructor of a Struct

- Rust 에서는 `new`라는 단어를 사용하여 생성자 메서드의 이름을 지정하는 것이 관례이지만 이것이 필수 사항은 아닙니다  
  구조체에 적합하다면 생성자 메서드에 대해 원하는 이름을 선택할 수 있습니다
- `new`는 Rust 의 키워드가 아닙니다. 이는 Rust 코드에서 일반적으로 사용되는 명명 규칙일 뿐입니다

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
}
```


```rust
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Method that borrows self immutably
    fn distance_from_origin(&self) -> f32 { ... }

    // Method that borrows self mutably
    fn translate(&mut self, dx: f32, dy: f32) { ... }

    // Method that takes ownership of self
    fn into_tuple(self) -> (f32, f32) { ... }

    // Associated function
    fn from_tuple(coords: &(f32, f32)) -> Point {
        Point { x: coords.0, y: coords.1 }
    }
}

fn main() {
  let tuple = (10, 20);
  let q = Point::from_tuple(&tuple);
  println!("Point from tuple: ({}, {})", q.x, q.y);
}
```

String의 함수에서 `new`를 호출한 것을 기억하시나요?
이는 표준 라이브러리 `String` 유형의 관련 함수이므로 문자열 인스턴스가 없어도 유형 이름에서 직접 호출할 수 있습니다

```rust
fn main() {
    let empty_string = String::new();

    let message = String::from("Hello");
}
```