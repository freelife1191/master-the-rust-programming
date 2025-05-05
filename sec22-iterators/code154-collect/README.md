# `collect()`

`collect()` 는 이터레이터를 받아 소비하는 소모성 이터레이터 메소드로, 모든 요소를 반복하여 `Vec`, `HashMap` 또는 다른 컬렉션 타입과 같은 컬렉션으로 수집함을 의미함

`collect()` 메소드를 사용할 때는, 가능한 컬렉션 타입이 많기 때문에 수집할 컬렉션의 타입을 명시하는 것이 중요함

```rust
let numbers = vec![1, 2, 3, 4, 5];

//Explicit type annotation for the variable
//변수에 대한 명시적 타입 명시
let number_strings: Vec<String> = numbers.iter().map(|&num| num.to_string()).collect();

//Turbofish syntax (::<>) to specify the type directly with the collect() method
//collect() 메소드에 직접 타입을 명시하는 터보피쉬 문법 (::<>)
let number_strings = numbers.iter().map(|&num| num.to_string()).collect::<Vec<String>>();
```

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    //Explicit type annotation for the variable
    // 변수에 대한 명시적 타입 명시
    //let number_strings: Vec<String> = numbers.iter().map(|&num| num.to_string()).collect();

    //turbofish syntax ::<>
    // 터보피쉬 문법 ::<>
    let number_strings = numbers.iter().map(|&num| num.to_string()).collect::<Vec<String>>();


    println!("{:?}", number_strings);
}
```

```bash
[1, 2, 3, 4, 5]
```

```rust
use std::collections::HashMap; // HashMap을 사용하기 위해 필요합니다

fn main() {
    let fruit_basket = vec![("apple", 1), ("banana", 2)];

    //lets collect vec of tuples into HashMap
    // 튜플 벡터를 HashMap으로 수집해 봅시다
    let fruits_map: HashMap<_, _> = fruit_basket.into_iter().collect();

    println!("{:?}", fruits_map);
}
```

```bash
error[E0412]: cannot find type `HashMap` in this scope
  --> src/main.rs:10:21
   |
10 |     let fruits_map: HashMap<_, _> = fruit_basket.into_iter().collect();
   |                     ^^^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
4  + use ahash::HashMap;
   |
4  + use hashbrown::HashMap;
   |
4  + use hashbrown @ 12 3::HashMap; // Version might vary
   |
4  + use nom::lib::std::collections::HashMap;
   |
   and 1 other candidate // Usually std::collections::HashMap

For more information about this error, try `rustc --explain E0412`.
error: could not compile `playground` (bin "playground") due to previous error
```

```rust
use std::collections::HashMap;

fn main() {
    let fruit_basket = vec![("apple", 1), ("banana", 2)];

    //lets collect vec of tuples into HashMap
    // 튜플 벡터를 HashMap으로 수집해 봅시다
    let fruits_map: HashMap<&str, i32> = fruit_basket.into_iter().collect();

    println!("{:?}", fruits_map);
}
```

```bash
{"banana": 2, "apple": 1}
```

```rust
use std::collections::HashMap;

fn main() {
    let fruit_basket = vec![("apple", 1), ("banana", 2)];

    //lets collect vec of tuples into HashMap
    // 튜플 벡터를 HashMap으로 수집해 봅시다
    let fruits_map: HashMap<&str, i32> = fruit_basket.into_iter().collect();

    println!("{:?}", fruits_map);
}
```

```bash
{"apple": 1, "banana": 2}
```


```rust
fn main() {
    let chars = vec!['r', 'u', 's', 't'];

    let word: String = chars.into_iter().collect();

    println!("{}", word);
}
```

```bash
rust
```

```rust
fn main() {
    let fruit_basket = vec![("apple", 1), ("banana", 2)];

    //lets collect vec of tuples into HashMap
    // 튜플 벡터를 HashMap으로 수집해 봅시다
    let fruits_map: Vec<_> = fruit_basket.into_iter().collect();

    println!("{:?}", fruits_map);
}
```

```bash
[("apple", 1), ("banana", 2)]
```

## `collect()`

* `collect()` 메소드는 항상 이터레이터에 대해 호출됨
* `collect()` 는 이터레이터를 소비함 이터레이터에 대해 `collect()` 를 호출하고 나면, 컬렉션을 만드는 데 이터레이터가 '소진(used up)'되었으므로 해당 이터레이터를 다시 사용할 수 없음
* `collect()` 가 아이템을 컬렉션으로 모으는 능력은 `FromIterator` 트레잇에 의해 제어됨 아이템을 수집하려는 컬렉션 타입은 이터레이터가 생성하는 아이템 타입에 대해 `FromIterator` 트레잇을 구현해야 함
* `collect()` 는 일반적으로 부수 효과가 없는 속성 때문에 사용됨 전역 상태를 수정하거나 `I/O` 작업을 수행하는 등의 작업을 수행하기보다는 데이터 수집을 목적으로 함