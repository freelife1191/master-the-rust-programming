## 구조체의 메서드 및 관련 기능

### Methods
- 메소드는 구조체 또는 열거형의 특정 인스턴스와 연결됩니다
- 메소드는 구조체 또는 열거형에 대한 impl 블록 내에서 정의되며 첫 번째 매개변수는 항상 구조체 또는 열거형의 인스턴스에 대한 참조입니다
- 구조체 메서드는 점(.) 연산자를 사용하여 액세스합니다
  예를 들어, `person`이라는 이름의 `Person` 구조체 인스턴스가 `Introduction` 이라는 메서드를 가지고 있다면 `person.introduce()`로 접근할 수 있습니다

### Associated functions

- 관련 함수는 특정 인스턴스에 연결되지 않으며 유형 자체에서 호출됩니다
- 관련 함수는 impl 블록 내에서도 정의되지만 첫 번째 매개변수는 유형에 대한 참조가 아닙니다
- 관련 함수는 이중 콜론(`::`) 연산자를 사용하여 액세스됩니다
- 예를 들어 `new()` 함수와 연관된 `Person`이라는 구조체가 있는 경우 `Person::new()`로 액세스할 수 있습니다


```
// Struct definition
struct MyStruct {
    ....
}

// f1() and f2 are methods/associated functions of 'MyStruct'
// Put them inside impl <Struct-Name> block
impl MyStruct {
    fn f1(...) { .... }
    fn f2(...) { .... }
}

fn main() {
    // Create an instance of 'MyStruct'
    let struct_inst = MyStruct { .... };

    // Method call
    struct_inst.f1();
}
```

구조체 메서드의 `self` 키워드는 메서드가 호출되는 구조체의 인스턴스를 나타냅니다  
이는 다른 많은 객체지향 언어의 'this'와 유사합니다


```rust
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    // Method that borrows self immutably
    fn distance_from_origin(&self) -> f32 { // 약식 표기법 `self: &Point`
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let mut p = Point { x: 3.0, y: 4.0 };

    let dist = p.distance_from_origin();
    println!("Distance from origin: {}", dist);
}
```

- `self`를 메소드의 첫 번째 매개변수로 사용하는 것은 메소드가 인스턴스의 소유권을 가져와 이를 소비하여 다른 것으로 변환한다는 의미입니다  
- 이는 호출자가 호출 후 원본 인스턴스를 사용하는 것을 방지하려는 경우에 유용합니다

```rust
#[derive(Debug)]
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
    fn into_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

fn main() {
    let mut p = Point { x: 3.0, y: 4.0 };

    let tuple = p.into_tuple();
    println!("Point as tuple: {:?}", tuple);
    
    println!("{:?}", p); // Error
}
```