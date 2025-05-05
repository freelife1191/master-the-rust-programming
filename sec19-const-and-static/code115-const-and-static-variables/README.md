## Constant in Rust

```rust
const MAX_VALUE: u32 = 100;
const MESSAGE: &str = "Hello, world!";

fn main() {
    let ref_to_const = &MAX_VALUE;
    println!("The maximum value is: {}", MAX_VALUE);
    println!("MESSAGE: {}", MESSAGE);
    println!("value: {}", ref_to_const);
}
```

다음은 Rust 상수의 몇 가지 특징이다:

1. Immutable: 상수는 변경할 수 없다. 즉, 일단 할당되면 해당 값을 수정할 수 없다. 이렇게 하면 프로그램이 실행되는 동안 값이 일정하게 유지된다.
2. Known at complie time: 상수 값은 컴파일 타임에 계산 가능해야 한다. 이를 통해 Rust 컴파일러는 프로그램을 최적화하고 상수 값이 사용되는 곳마다 직접 대체할 수 있다.
3. Scoped globally: 상수는 전역적으로 사용할 수 있다. 즉, 프로그램 어디에서나 상수에 접근할 수 있다.
4. Type annotation: 상수는 타입을 명시해야 한다. 이는 Rust 컴파일러가 상수의 타입을 알 수 있게 한다.
5. Naming convention: 상수 이름은 대문자로 작성하며, 단어 사이에 밑줄을 사용한다.
6. No mutability: 상수는 변경할 수 없다. 따라서 `mut` 키워드를 사용할 수 없다.
7. No shadowing: 상수는 변수와 달리 shadowing이 불가능하다. 즉, 같은 이름의 상수를 다시 선언할 수 없다.
8. No memory allocation: 상수는 런타임에 메모리를 할당하지 않는다. 따라서 상수는 스택이나 힙에 저장되지 않고 컴파일 타임에 상수 값이 직접 대체된다.
9. No function call: 상수는 함수 호출을 포함할 수 없다. 즉, 상수는 함수 호출을 포함할 수 없다.
11. Inlined: 상수는 사용되는 곳마다 직접 대체된다. 따라서 상수는 프로그램의 크기를 늘리지 않는다.


### Const lifetime

### Static items in Rust

```rust
static GLOBAL_VALUE: i32 = 42;

// Non mutable static variable
fn main() {
    println!("GLOBAL_VALUE: {}", GLOBAL_VALUE);
}
```

### Mutable static value

```rust
static mut MUTABLE_STATIC: i32 = 42;

fn main() {
    unsafe {
        // Modifying mutable static requires an unsafe block
        MUTABLE_STATIC = 10;
        println!("updated mutable static: {}", MUTABLE_STATIC);
    }
}
```