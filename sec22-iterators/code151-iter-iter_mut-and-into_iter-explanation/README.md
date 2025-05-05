# ``iter()``, ``iter_mut()`` 그리고 ``into_iter()``

``iter()``, ``iter_mut()``, 그리고 ``into_iter()`` 는 Rust에서 이터레이터로 작업할 때 가장 자주 사용되는 메소드 중 일부이며, 특히 다음과 같은 컬렉션의 컨텍스트에서 그러함,

* ✅ `Vec<T>`
* ✅ `VecDeque<T>`
* ✅ `LinkedList<T>`
* ✅ `arrays ([T; N])`
* ✅ `HashMap<K, V>`
* ✅ `slices (&[T] and &mut [T])`

## `iter()`

* **사용법**:
    * `iter()` 는 컬렉션의 요소에 대한 불변 참조(`&T`)를 생성하는 이터레이터를 만드는 데 사용됨
* **사용 시점**:
    * 컬렉션을 변경하지 않고 그대로 두면서 요소 읽기에만 관심이 있을 때 `iter()` 를 사용함
    * 데이터 복제(cloning)를 피하고 싶거나 복제할 수 없는 타입으로 작업할 때 사용함

## `iter_mut()`

* **사용법**:
  * `iter_mut()` 는 컬렉션의 요소에 대한 가변 참조(`&mut T`)를 생성하는 이터레이터를 만드는 데 사용됨
* **사용 시점**:
  * 컬렉션의 요소를 제자리에서 수정해야 할 때 `iter_mut()` 를 사용함

## `into_iter()`

* **사용법**:
  * `into_iter()` 는 컬렉션의 소유권을 가져와서 소유된 값 (T)을 생성하는 이터레이터를 만드는 데 사용됨
* **사용 시점**:
  * 컬렉션을 소비하고 반복 후에 원본 컬렉션이 더 이상 필요하지 않을 때 `into_iter()` 를 사용함 이는 컬렉션을 다른 타입으로 변환하거나 요소 복제를 피하고 싶을 때 특히 유용함

## Rust 표준 컬렉션의 이터레이터 메소드

1. `Vec (Vector)`
2. `VecDeque`
3. `LinkedList`
  * `iter()`: `&T` 에 대해 반복함
  * `iter_mut()`: `&mut T` 에 대해 반복함
  * `into_iter()`: 컬렉션을 소비하고 `T` 에 대해 반복함
4. `Arrays ([T; N])`
  * `iter()`: `&T` 에 대해 반복함
  * `iter_mut()`: `&mut T` 에 대해 반복함
  * `into_iter()`: 소유된 값 (`T`) 에 대해 반복함 `T` 가 `non-Copy` 면 배열을 소비하지만, `T` 가 `Copy` 타입이면 복사된 값들에 대해 반복하고 배열은 그대로 유지됨

**`HashMap<K, V>`**

* `iter()`: (`&K`, `&V`) 에 대해 반복하며, 키와 값에 대한 불변 참조를 제공함
* `iter_mut()`: (`&K`, `&mut V`) 에 대해 반복하며, 값에 대해서는 가변 접근을 허용하지만 키에 대해서는 허용하지 않음
* `into_iter()`: `HashMap` 을 소비하고 소유된 키-값 쌍 (K, V) 에 대해 반복함

**`Slices (&[T] and &mut [T])`**

* `iter()`: `&[T]` 또는 `&mut [T]` 에 대해 `&T` 로 반복함
* `iter_mut()`: `&mut [T]` 에 대해 `&mut T` 로 반복함
* `into_iter()`: 소유된 아이템을 소비하고 생성하는 대신 참조 (`&T` 또는 `&mut T`)를 생성함

`String` 의 경우, 표준 이터레이터 메소드 (`iter()`, `iter_mut()`, `into_iter()`) 는 일반적인 컬렉션 의미로 사용되지 않음 대신, 반복 목적으로 `chars()` 나 `bytes()` 와 같은 특화된 메소드가 사용됨

**`String:`**

* `iter()`: 직접 적용 불가 유니코드 스칼라 값 (문자)을 반복하려면 `chars()` 를 사용하거나 바이트 (`u8`)를 반복하려면 `bytes()` 를 사용함
* `iter_mut()`: 적용 불가 `String` 은 문자나 바이트를 가변적으로 반복할 방법을 제공하지 않음
* `into_iter()`: `String` 에 `into_iter()` 를 직접 사용하는 것은 표준이 아님 대신, `into_bytes()` 를 사용하여 `String` 을 바이트 벡터로 변환함 이는 `String` 을 소비하고 바이트 (`u8`) 에 대한 반복을 허용함

```rust
fn main() {
  let msg = "Good Morning".to_string();

  let vec_of_bytes: Vec<_> = msg.bytes().collect();

  println!("{:?}", vec_of_bytes);
}
```

```bash
$ cargo run
[71, 111, 111, 100, 32, 77, 111, 114, 110, 105, 110, 103]
```

```rust
fn main() {
    let msg = "Good Morning".to_string();

    let vec_of_chars: Vec<_> = msg.chars().collect();

    println!("{:?}", vec_of_chars);
}
```

```bash
['G', 'o', 'o', 'd', ' ', 'M', 'o', 'r', 'n', 'i', 'n', 'g']
```

**`&str` (문자열 슬라이스):**

* `iter()`: 직접 적용 불가 대신, 유니코드 스칼라 값 (문자)을 반복하려면 `chars()` 를 사용하거나 바이트 (`u8`)를 반복하려면 `bytes()` 를 사용함
* `iter_mut()`: 적용 불가 `&str` 은 문자열에 대한 불변 참조이므로 가변 반복을 허용하지 않음
* `into_iter()`: `&str` 에 `into_iter()` 를 직접 사용하는 것은 적용 불가 문자를 반복하려면 `chars()` 를 사용하고, 바이트를 반복하려면 `bytes()` 를 사용함 두 메소드 모두 각각의 요소에 대한 이터레이터를 반환함