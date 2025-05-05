## retain() and retain_mut()

이러한 함수를 사용하면 주어진 조건을 만족하는 벡터 요소만 유지하고 다른 요소는 모두 제거할 수 있습니다  
본질적으로 이는 `Vec`에 대한 내부 필터입니다


```rust
fn main() {
    let mut numbers = vec![-3, -2, -1, 0, 1, 2, 3];

    // This will retain only the positive numbers in the Vec
    numbers.retain(|x| *x > 0);
    
    println!("{:?}", numbers); // prints: [1, 2, 3]
}
```

벡터의 양수 요소만 유지합니다