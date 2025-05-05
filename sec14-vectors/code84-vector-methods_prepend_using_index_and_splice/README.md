## Is comparing vectors possible?

- `Vec<T>`는 벡터에 저장된 유형 T도 `PartialEq`를 구현하는 한 `PartialEq` 특성을 구현합니다
- 벡터 내부의 요소가 `PartialOrd` 특성(예: `i32`, `f32`, `String` 등)도 구현하는 경우 사전순으로 벡터를 비교할 수도 있습니다(사전에서 단어가 정렬되는 방식과 유사)

## Prepend

- Rust의 `Vec<T>`는 단방향으로 성장합니다. 즉, 끝에서만 확장되거나 축소됩니다. 벡터의 시작 주소는 고정된 상태로 유지되므로 벡터의 용량이 증가하더라도 첫 번째 요소의 주소는 변경되지 않습니다
- 직접 추가 작업은 지원되지 않지만 insert 메소드를 사용하여 `Vec<T>` 시작 부분에 요소를 삽입하여 효과적으로 추가 작업을 수행할 수 있습니다

## insert()

지정된 인덱스에 요소를 삽입하려면 `Vec<T>`에 삽입 메소드를 사용하여 모든 후속 요소를 꼬리쪽으로 효과적으로 밀어냅니다. 인덱스가 0인 insert를 사용하면 요소가 벡터 앞에 추가됩니다

```rust
fn main() {
    let mut vec = vec![1, 2, 3];
    vec.insert(0, 25);
    println!("{:?}", vec); // [25, 1, 2, 3]
}
```

시간 복잡도는 O(n)입니다. 여기서 n은 새 요소가 삽입되는 지정된 인덱스 오른쪽에 있는 요소 수를 나타냅니다


## Prepend using splice

- `splice()`를 사용하면 한 벡터 앞에 다른 벡터 요소를 추가할 수 있습니다.

```rust
fn main() {
    let mut main_vec = vec![4, 5, 6];
    let vec_to_prepend = vec![1, 2, 3];

    main_vec.splice(0..0, vec_to_prepend);

    println!("{:?}", main_vec); // prints: [1, 2, 3, 4, 5, 6]
}
```