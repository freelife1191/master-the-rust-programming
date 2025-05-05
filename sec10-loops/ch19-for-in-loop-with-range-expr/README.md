## for in

```
for element in collection {
    // 컬렉션의 각 요소에 대해 실행할 코드
}
```

- 'element' 는 루프가 반복될 때마다 컬렉션의 각 요소에 할당되는 변수입니다. 루프 내부의 코드는 컬렉션의 각 요소에 대해 한 번씩 실행됩니다
- 컬렉션은 array, vector, range 또는 Iterator trait 을 구현하는 모든 유형일 수 있습니다


### for loop with a range of numbers

```rust
fn main() {
    'outer: for i in 0.. { 
        println!("{}", i);
        if i == 10 {
          break 'outer;
        }
    }
}

fn main() {
    for i in 2..=5 {
        println!("{}", i);
    }
}
```

- `i`는 루프 변수입니다. 범위는 선언된 루프 블록으로 제한됩니다
- `i`는 범위의 값과 동일한 유형입니다
- 값이 있는 `break`는 for 루프와 함께 사용할 수 없습니다