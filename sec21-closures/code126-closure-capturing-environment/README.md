## Capturing Environment

클로저는 주변 환경에서 변수를 캡처하고 사용하는 고유한 기능을 가지고 있으며, 이를 통해 자체 범위 외부에서 정의된 변수를 참조하고 사용할 수 있습니다  
일반 함수는 인수로 전달되거나 자체 범위 내에서 정의된 변수에만 액세스하도록 제한됩니다

## Closures without Capturing an Environment

```rust
fn main() {
    // 클로저 'add'는 주변 환경에서 어떤 변수도 캡처하지 않습니다
    let add = |a, b| a + b;

    let result1 = add(2, 3);
    println!("{}", result1); // Output: 5

    // "Surrounding environment" 주요 기능의 범위를 나타냅니다
    let result2 = add(5, 7u32);
    println!("{}", result2); // Output: 12
}
```


## Closure Capturing an Environment

```rust
fn main() {
    let x = 10; // 외부 범위의 변수

    // 외부 환경에서 'x'를 캡처하는 클로저
    let add = |a| a + x;

    println!("Result 1: {}", add(2)); // 클로저는 외부 범위의 'x'를 사용합니다: 2 + 10 = 12

    let result1 = add(5); // 클로저는 외부 범위의 'x'를 사용합니다: 5 + 10 = 15
    println!("Result 2: {}", result1);
}
```

## Closure Capturing an Environment

1. Read-only (Immutable) Capture
2. Mutable Capture
3. Transfer of ownership to a closure

# Read-only (Immutable) Capture

```rust
fn main() {
    let x = 10;

    // The closure borrows 'x' immutably.
    let print = || println!("Read x: {}", x);

    print(); // Output: Read x: 10
    print(); // Output: Read x: 10
}
```

## Mutable Capture

클로저가 캡처된 변수(변경 가능한 액세스)를 수정해야 하는 경우 변수를 변경 가능하게 빌려옵니다  
이 경우 클로저는 일시적으로 변수에 대한 변경 가능한 참조를 보유하지만 소유권은 여전히 원래 범위에 유지됩니다  
클로저는 캡처된 변수의 값을 수정할 수 있습니다

```rust
fn main() {
    let mut x = 10;
    {
        // 클로저는 'x'를 가변적으로 빌려 수정합니다
        let mut print = || {
            x += 1;
            println!("Modified x: {}", x);
        };
        print(); // "Modified x: 11"
    }
    // 클로저를 여러 번 호출하고 'x'를 수정할 수 있습니다
}
```

## Rust의 변수 범위와 클로저

클로저 `print`는 단순한 참조가 아닌 소유권을 갖기 때문에 자체 `x` 복사본을 갖습니다  
이 소유 `x`의 수명은 이제 `x`가 원래 정의된 범위가 아니라 클로저에 연결됩니다

```rust
fn main() {
    // 'x'의 수명을 제한하는 새 범위를 만듭니다
    let mut x = 10;
    {
        // 클로저 'print'를 정의하고 'move' 키워드로 'x'를 캡처합니다
        let mut print = move || {
            // 클로저는 'x'의 소유권을 가져와 이를 변경합니다
            x += 1;
            println!("Modified x: {}", x);
        };
        print(); // "Modified x: 11"
    }
    // 범위는 여기서 끝나고 'x'는 범위를 벗어나 삭제됩니다
    print(); // 클로저는 여전히 여러 번 호출될 수 있으며 'x'를 수정할 수 있습니다
}
```

## Ownership and Borrowing in Rust

`i32`처럼 변수가 복사 특성을 구현하는 경우 이러한 소유권 이전은 전통적인 의미의 이동이 아닌 값의 복사본을 효과적으로 생성합니다

```rust
fn main() {
    let mut x = 10;
    let mut print = move || {
        x += 1;
        println!("Modified x: {}", x);
    };
    print(); // Output: Modified x: 11
    println!("{}", x); // Output: 10
}
```