# Iterators

1. `추상화(Abstraction)`: 반복자는 컬렉션의 기본 세부 정보를 추상화하여 프로그래머가 데이터 액세스의 세부 사항보다는 수행하려는 작업에 집중할 수 있도록 합니다.
2. `안전성(Safety)`: Rust의 반복자는 범위를 벗어난 오류와 같은 일반적인 실수를 방지합니다. 컬렉션의 한도를 넘어서는 데이터에 액세스할 위험 없이 안전한 탐색을 보장합니다.
3. `효율성(Efficiency)`: 반복자는 성능에 최적화되어 있습니다. 반복자 연결은 여러 작업을 단일 루프로 결합하여 오버헤드와 메모리 할당을 줄일 수 있습니다.
4. `일관성(Uniformity)`: 반복자를 사용하면 동일한 메서드 세트(예: 맵, 필터 등)를 다양한 데이터 구조에 적용하여 다양한 컬렉션에서 데이터 조작을 위한 일관된 인터페이스를 제공할 수 있습니다. 이러한 일관성은 코드 재사용성과 가독성에 도움이 됩니다.


## Trait 'Iterator'

Rust 프로그래밍 언어에서 반복자는 반복자 특성을 구현하는 모든 유형이라고 말할 수 있습니다.

https://doc.rust-lang.org/std/iter/trait.Iterator.html


### Simple example iterator in action

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];
    for value in arr {
        println!("{}", value);
    }
}
```

Output:

```shell
1
2
3
4
5
```

- 배열은 **IntoIterator** 특성을 구현합니다. 이는 for 루프에서 직접 배열을 사용할 수 있음을 의미합니다.
- 배열과 함께 for 루프를 사용하면 Rust는 자동으로 배열의 `into_iter()`를 호출하여 이를 반복자로 변환합니다.

https://doc.rust-lang.org/std/iter/trait.IntoIterator.html


### Primitive Type array Trait Implementations IntoIterator


`impl<'a, T, const N: usize> IntoIterator for &'a [T; N]`

이 구현은 배열에 대한 불변 참조가 있고 반복자를 생성하려는 경우에 사용됩니다.  
배열 요소에 대한 불변 참조를 생성하는 반복자를 반환합니다.

`impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N]`

이 구현은 배열에 대한 변경 가능한 참조가 있고 반복자를 생성하려는 경우에 사용됩니다.  
배열 요소에 대한 변경 가능한 참조를 생성하는 반복자를 반환합니다.

`impl<T, const N: usize> IntoIterator for [T; N]`

이 구현은 참조가 아닌 배열이 있고 반복자를 생성하려는 경우에 사용됩니다.  
배열의 요소를 직접 생성하는 반복자를 반환합니다.  
`T`가 Copy를 구현하지 않는 한 배열을 소비합니다.

