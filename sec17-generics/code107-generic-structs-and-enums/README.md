## Generics with struct

```rust
#[derive(Debug)]
struct Pair<T> { // 이를 통해 구조체는 필드, 메서드 및 구현에서 유형 매개변수 `T`를 사용할 수 있습니다
    first: T,
    second: T,
}

fn main() {
    let pair_of_ints = Pair { first: 1, second: 2 };
    println!("{:?}", pair_of_ints);

    let pair_of_strings = Pair { first: "hello", second: "world" };
    println!("{:?}", pair_of_strings);

    println!("{:?}", Pair { first: "?", second: "?" });
}
```

## 2개의 일반 유형 매개변수가 있는 구조체

```rust
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let pair_of_ints = Pair { first: 1, second: 2 };
    let pair_of_strings = Pair { first: "hello", second: "world" };

    println!("{:?}", pair_of_ints);
    println!("{:?}", pair_of_strings);
}
```

### Methods of generic type struct

`impl` 뒤의 유형 매개변수는 `Pair<T>`의 인스턴스화에 작동하는 메소드를 구현하고 있음을 나타냅니다  
여기서 `T`는 모든 유형이 될 수 있습니다

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    
}
```

이는 함수 시그니처에 사용할 수 있는 `T` 라는 일반 유형 매개변수를 지정합니다

> Generic 수명 및 Generic 유형 매개변수는 꺾쇠괄호 <>를 사용하여 지정됩니다

```rust
// 이는 일반 유형 T의 일부인 &[T] 유형의 'v' 라는 매개변수를 지정합니다
fn find_max<T>(v: &[T]) -> Option<T> // 특성의 옵션 값은 T로 제한됩니다
    where T: std::cmp::PartialOrd + Copy,
    // generic function 와 함께 사용할 수 있는 유형을 제한하는 Trait bounds
{
    if v.is_empty() {
        return None;
    }
    let mut max = v[0];
    for &n in v {
        if n > max {
            max = n;
        }
    }
    Some(max)
}
```


## Mix of generic and concrete methods

```rust
impl<T> Pair<T> {
    fn new(x: T, y: T, label: &str) -> Pair<T> {
        Pair {
            x,
            y,
            label: String::from(label),
        }
    }
    
    fn get_label(&self) -> &str {
        &self.label
    }
    
    fn set_label(&mut self, new_label: &str) {
        self.label = String::from(new_label);
    }
}

impl Pair<i32> {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}

fn main() {
    let mut pair_i32 = Pair::new(3, 5, "Integer Pair");
    println!("{} + {} = {}", pair_i32.x, pair_i32.y, pair_i32.sum());
    
    let pair_str = Pair::new("Hello", "Rust", "String Pair");
    println!("Label: {}", pair_str.get_label());
    
    pair_i32.set_label("New Label");
    println!("Label: {}", pair_i32.get_label());
}
```

## The `Self` keyword

> 대문자 "S"가 있는 Self 는 구현 대상 유형을 나타내고, 소문자 "s"가 있는 self 는 해당 유형의 인스턴스를 나타냅니다
> 
```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    // 여기서 `Self`는 메소드가 구현되는 유형인 `Point<T>` 유형을 나타냅니다
    fn new(x: T, y: T) -> Point<T> { // Point<T> -> Self
        Point { x, y }
    }

    // 여기서 `Self`는 메소드가 구현되는 유형인 `Point<T>` 유형을 나타냅니다
    fn distance(&self, other: &Point<T>) -> f64 // &Point<T> -> &Self
    where
        T: std::ops::Sub<Output=T>
        + std::ops::Mul<Output=T>
        + std::ops::Add<Output=T>
        + std::convert::Into<f64> + Copy
    {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let sum = (dx * dx) + (dy * dy);
        (sum.into() as f64).sqrt()
    }
}

fn main() {
    let p1 = Point::new(3, 4);
    let p2 = Point::new(6, 8);
    let distance = p1.distance(&p2);
    println!("Distance between {:?} and {:?} is {}", p1, p2, distance);
}
```

## Generics with enum

```rust
/*
MyEnum은 T 유형으로 매개변수화된 일반 열거형이다
 */
enum MyEnum<T> {
    A(T),
    B(T),
}

impl<T> MyEnum<T> {
    fn get(&self) -> &T {
        match self {
            MyEnum::A(t) => t,
            MyEnum::B(t) => t,
        }
    }

    fn set(&mut self, t: T) {
        match self {
            /*
            ref mut x, 참조로 값을 일치시킨 다음 변경 가능한 방식으로 수정할 수 있다
             */
            MyEnum::A(ref mut x) => *x = t,
            MyEnum::B(ref mut x) => *x = t,
        }
    }
}

fn main() {
    /*
    MyEnum::A(5)는 MyEnum<T> 열거형의 구체적인 인스턴스를 만든다
    e1의 유형은 컴파일러에 의해 MyEnum<i32>로 추론된다
     */
    let mut e1 = MyEnum::A(5);
    let e2 = MyEnum::B("Hello");
    println!("e1.get(): {}", e1.get());
    println!("e2.get(): {}", e2.get());
    e1.set(10);
    println!("e1.get() after set(): {}", e1.get());
}
```