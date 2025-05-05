### 동일한 유형 `T` 의 두 유형 매개변수가 있는 Generic function

```rust
fn combine<T: Clone, T: 'static>(a: T, b: T) -> Vec<T> {
    let mut v = Vec::new();
    v.push(a);
    v.push(b);
    v
}
```

```rust
fn main() {
    let v1 = combine(3, 4);
    println!("{:?}", v1); // Output: [3, 4]

    let v2 = combine("hello".to_string(), "world".to_string());
    println!("{:?}", v2); // Output: ["hello", "world"]
}
```

### 두 개의 서로 다른 유형 매개변수 `T`와 `U`를 사용하는 Generic function

```rust
fn combine<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

fn main() {
    let t1 = combine(3, "three");
    // Output: (3, "three")
    let t2 = combine("two", 2.0);
    // Output: ("two", 2.0)
    let t3 = combine("one".to_string(), "two".to_string());
    // Output: ("one", "two")
}
```