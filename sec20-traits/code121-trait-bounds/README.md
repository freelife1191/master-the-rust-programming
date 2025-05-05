# Trait bounds

```rust
impl<T> Rectangle<T> {
    fn area(&self) -> T {
        /*
        `area` 메소드에 사용된 곱셈 연산(*)은 제네릭 유형 `T`에 직접 적용할 수 없다
        곱셈 연산에는 `std::ops::Mul` 특성을 구현하기 위해 `T` 유형이 필요하다. 이는 가능한 모든 유형 `T`에 대해 보장되지는 않는다
        
        https://doc.rust-lang.org/stable/std/ops/trait.Mul.html
        https://doc.rust-lang.org/stable/std/ops/trait.Mul.html#associatedtype.Output
         */
        self.width * self.height
    }
}

struct Rectangle<T> {
    width: T,
    height: T,
}

fn main() {
    let rect1 = Rectangle { width: 5.0, height: 10.0 };
    let rect2 = Rectangle { width: 7.5, height: 3.2 };
    
    println!("Area of rect1: {}", rect1.area());
    println!("Area of rect2: {}", rect2.area());
}
```

### `where` clause

일반 유형 매개변수에 대해 여러 특성 범위를 지정할 때 `where` 절을 사용하면 명확성과 가독성을 높일 수 있다
`where` 절을 사용하면 특성 범위를 코드의 나머지 부분과 분리하고 요구 사항에 대한 보다 명확한 정의를 제공할 수 있다

### Disambiguating between traits

- 다른 모듈에 정의된 특성을 사용하려는 경우 일반적으로 전체 경로를 사용하여 특성을 지정해야 합니다. 이는 서로 다른 모듈이나 상자에 존재할 수 있는 동일한 이름을 가진 여러 특성을 명확하게 하기 위한 것입니다.
- 예를 들어 `Display` 특성은 `std::fmt` 모듈에 정의되어 있으므로 코드에서 이를 사용하려면 전체 경로 `std::fmt::Display`를 지정해야 합니다. 이는 특히 서로 다른 모듈에 정의된 여러 개의 `Display` 특성이 있을 수 있는 경우 Rust 컴파일러가 사용자가 참조하는 `Display` 특성을 이해하는 데 도움이 됩니다.
- 단지 `Display` 대신 `fmt::Display`를 사용하면 어떤 `Display` 특성을 구현하거나 사용하고 있는지 명시적으로 알 수 있어 혼란을 방지하고 올바른 특성이 사용되도록 보장할 수 있습니다.
- 특성이 Rust 서문에 있는 경우 이를 구현할 때 전체 경로를 사용할 필요가 없습니다. 예를 들어 `PartialOrd` 특성은 Rust 서문의 일부이므로 이를 구현하기 위해 전체 경로를 사용할 필요가 없습니다.