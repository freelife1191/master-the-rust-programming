## What is a closure?

- 클로저는 해당 환경에서 값을 캡처할 수 있는 익명 함수입니다
- 클로저는 Rust에서 간결하고 재사용 가능한 코드를 작성하기 위한 강력한 도구입니다. 다음과 같은 용도로 사용할 수 있습니다
    - 함수를 다른 함수에 인수로 전달
    - 함수에서 함수를 반환
    - 컬렉션 반복
    - 이벤트 핸들러 작성
    - 사용자 정의 반복자 만들기
    - 함수를 변수에 저장

## Closure syntax

매개변수는 쉼표로 구분된 임의 개수의 변수일 수 있으며 모든 유형이 될 수 있습니다. 클로저의 본문은 실행하려는 모든 코드가 될 수 있습니다

Rust의 클로저는 `| 파이프를 사용하여 정의됩니다. ... |` 매개변수를 지정하는 구문:

```
| parameters | -> return-type {
    // body of closure
}
```

컴파일러는 대부분의 경우 매개변수와 반환 유형을 추론할 수 있기 때문에 클로저 정의에서 매개변수와 반환 유형을 언급할 필요가 없는 경우가 많습니다

```rust
fn main() {
    // Define a closure that takes an `i32` parameter `x` and returns `x + 1`.
    let closure = |x: i32| {
        return x + 1;
    };

    // Call the closure with the argument `10`.
    let result = closure(10);
    println!("{}", result);
}
```

```rust
fn main() {
  let get_z = |x: i32| {
    // A closure with multi-line body
    let y = x + 1;
    let z = y * 2;
    z
  };
  println!("{}", get_z(10))
}
```

### Closure with multiple parameters

```rust
fn main()  {
  // Closure with two parameters: x and y
  // 주어진 클로저에 대해 동등한 명명된 함수
  let add = |x: i32, y: i32| x + y;
  
  let result = add(5, 10);
  println!("Result: {}", result);
}
```

## Conciseness

클로저는 인라인으로 정의할 수 있기 때문에 간결한 코드를 작성하는 데 사용할 수 있습니다  
이는 주어진 클로저에 대해 별도의 명명된 함수를 정의할 필요 없이 다른 함수의 본문 내에서 클로저를 정의할 수 있음을 의미합니다

## 클로저 매개변수의 유형 추론

Rust 클로저에는 항상 유형 주석이 필요한 것은 아닙니다  
Rust 컴파일러의 유형 추론 시스템은 종종 클로저 매개변수의 유형을 추론하고 클로저 사용 방법에 따라 자동으로 값을 반환할 수 있습니다  

### 명시적 유형 주석의 예

```rust
fn main() {
    // 매개변수 및 반환 유형에 대한 명시적 유형 주석
    let add = |x: i32, y: i32| -> i32 { x + y };
    let result = add(5, 10);
    println!("Result: {}", result);
}
```

```rust
fn main() {
    let add = |x, y| x + y;
    // 컴파일러는 'x'와 'y'의 유형을 `i32`로 추론할 수 있습니다
    let result = add(5, 10);
    println!("Result: {}", result);
}
```

## 후속 호출에서 다른 유형의 클로저를 사용하려고 시도합니다

매개변수에 대한 명시적인 유형 주석을 제공하지 않고 클로저를 정의하면 Rust는 클로저의 첫 번째 사용에서 유형을 추론하려고 시도합니다  
유형 추론이 완료되면 컴파일러는 해당 클로저의 모든 후속 사용이 해당 매개변수에 대해 동일한 유형을 가질 것으로 기대합니다