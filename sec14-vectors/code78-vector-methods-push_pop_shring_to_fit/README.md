## push()

`push` 메소드는 `Vec` 끝에 요소를 추가하는 데 사용됩니다. 'Vec'의 길이가 하나씩 늘어납니다

```rust
fn main() {
    let mut numbers = Vec::new();

    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers); // Outputs: [1, 2, 3]
}
```

## pop()

`Vec`에서 `pop()`을 호출하면 `Option<T>`가 반환됩니다. 여기서 `T`는 벡터에 저장된 요소의 유형입니다

벡터가 비어 있지 않으면 `pop()`은 마지막 요소를 제거하고 `Some(value)`를 반환하며 벡터의 길이를 1만큼 줄입니다. 벡터가 비어 있으면 `pop()`은 `None`을 반환합니다

`Vec`은 `pop()`마다 축소됩니까?  
길이 측면에서는(`len()`), 용량 측면에서는(`capacity()`) 측면에서는 그렇지 않습니다



## shrink_to_fit()

길이에 맞게 `Vec`의 용량을 줄이고 싶다면 `shrink_to_fit()` 메소드를 사용할 수 있습니다