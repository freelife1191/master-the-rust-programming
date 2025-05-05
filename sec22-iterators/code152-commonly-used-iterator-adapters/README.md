# 자주 사용되는 유용한 이터레이터 어댑터

* ✨ `map`: 제공된 함수로 각 요소를 변환함
* ✨ `filter`: 술어(predicate)를 만족하는 요소만 유지함
* ✨ `enumerate`: 각 아이템에 카운터를 추가하여 튜플을 생성함
* ✨ `flat_map`: 각 요소를 이터레이터로 매핑한 다음, 평탄화(flatten)함
* ✨ `take`: 처음 n개의 요소로 제한함
* ✨ `skip`: 처음 n개의 요소를 건너뜀
* ✨ `fold`: 요소들에 함수를 적용하여 이터레이터를 단일 값으로 축소(reduce)함
* ✨ `zip`: 두 이터레이터의 요소들을 짝지어 튜플로 만듦
* ✨ `chain`: 두 이터레이터를 하나로 연결함
* ✨ `filter_map`: `Option<T>` 를 반환하는 함수를 사용하여 요소를 필터링하고 매핑함
* ✨ `take_while`: 술어가 참인 동안 요소를 생성하다가 중지함
* ✨ `skip_while`: 술어가 참인 동안 요소를 건너뛰고 나머지를 생성함

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut doubled = Vec::new();

    for &number in &numbers {
        doubled.push(number * 2);
    }
    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
}
```

```bash
Original: [1, 2, 3, 4, 5]
Doubled: [2, 4, 6, 8, 10]
```

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    /*
    여기서 `map()` 은 각 요소가 원본 이터레이터의 각 요소에 
    `map` 에 명시된 함수를 적용한 결과인 새로운 이터레이터를 반환함
     */
    // Error
    // let mut doubled: Vec<_> = numbers.iter().map(|&n| n * 2).collect();
    let doubled: Vec<_> = numbers.iter().map(|&n| n * 2).collect();

    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
}
```

```bash
Compiling playground v0.0.1 (/playground)
warning: variable does not need to be mutable
  --> src/main.rs:10:9
   |
10 |     let mut doubled: Vec<_> = numbers.iter().map(|&n| n * 2).collect();
   |         ----^^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `playground` (bin "playground") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in ...s
     Running `target/debug/playground`
```

Success

```bash
Original: [1, 2, 3, 4, 5]
Doubled: [2, 4, 6, 8, 10]
```