# 이터레이터(Iterator)의 메소드들

Iterator 메소드는 크게 두 가지 타입으로 분류될 수 있음

* 이터레이터 어댑터 (Iterator adaptors)
* 소모성 이터레이터 (Consuming Iterators)

## 이터레이터 어댑터 (Iterator adapters)

* 이터레이터 어댑터는 이터레이터를 다른 종류의 이터레이터로 변환하는 메소드임 이들은 지연(lazy) 평가되며, 소모성 이터레이터로 소비하기 전까지는 아무 작업도 하지 않음
* 복잡한 이터레이터 연산 체인을 생성할 수 있게 함
* 이터레이터 어댑터는 기존 이터레이터를 받아 일부 수정이나 추가 기능이 적용된 새로운 이터레이터를 생성함
* 예시로는 ``map()``, ``filter()``, ``skip()``, ``take()``, ``enumerate()``, ``zip()`` 등이 있음

### 소모성 이터레이터 (Consuming Iterators)

* 소모성 이터레이터는 결과를 생성하기 위해 이터레이터를 소비하는 메소드임
* 소모성 이터레이터가 호출되면 이터레이터를 다 사용해 버리므로, 소모성 메소드를 호출한 후에는 해당 이터레이터를 다시 사용할 수 없음
* 이러한 메소드들은 일반적으로 전체 이터레이터를 반복하며 결과를 수집하거나 계산함
* 예시로는 ``collect()``, ``sum()``, ``max()``, ``count()`` 등이 있음

```rust
//consuming iterators
fn main() {
    let mut numbers = [1, 2, 3, 4];

    for i in 0..numbers.len() {
        numbers[i] *= numbers[i];
    }

    println!("{:?}", numbers);
}
```

```bash
[1, 4, 9, 16]
```

## ``for_each()``

이터레이터와 함께 ``for_each`` 를 사용할 때, 반복 중 각 아이템에 대해 수행할 작업을 정의하는 클로저(closure)를 제공함 ``for_each`` 메소드는 이 클로저를 이터레이터의 모든 아이템에 적용함

```rust
iterator.for_each(|item| {
   // Code to execute for each item
   // 각 아이템에 대해 실행할 코드
});
```

* ``iterator`` 는 작업 중인 이터레이터임
* ``for_each`` 는 이터레이터에 대해 호출되는 메소드임
* ``|item|`` 은 이터레이터로부터 각 아이템을 받는 클로저임
* 중괄호 ``{}`` 는 각 아이템에 대해 실행될 코드를 포함함
* ``for_each`` 에 전달하는 클로저는 계산 수행, 가변 참조인 경우 아이템 수정, 또는 외부 상태와의 상호작용 등 아이템으로 필요한 모든 작업을 수행할 수 있음

```rust
fn main() {
    //let mut numbers = [1, 2, 3, 4];
    let numbers = [1, 2, 3, 4];

    //type of number is &mut i32
    // numbers.iter_mut().for_each(|number| *number *= *number);
    numbers.iter().for_each(|number| {
        println!("{}", *number > 10);
    });
    
    println!("{:?}", numbers);
}

/*
1) numbers.iter_mut(): This creates a mutable iterator over the numbers array.
2) The iterator yields mutable references to each element in the array (&mut i32).
3) The closure |number| *number *= *number in the for_each call takes each mutable 
   reference, dereferences it with *number, and then squares the value.
*/
```

```bash
false
false 
false
false
[1, 2, 3, 4]
```

배열이나 벡터에 ``iter_mut()`` 를 사용하면, 요소들에 대한 가변 참조(``&mut T``)를 생성하는 이터레이터를 반환함 이를 통해 요소를 반복하면서 바로 수정할 수 있음