# `map()`

`map` 메소드는 입력 이터레이터를 받아, 주어진 클로저를 이 이터레이터의 각 요소에 적용하고, 출력 이터레이터를 생성함 출력 이터레이터의 각 요소는 입력 이터레이터의 해당 요소에 클로저를 적용한 결과임 이 과정은 클로저에 정의된 로직에 따라 각 요소를 원본 형태(입력 이터레이터에서)에서 새로운 형태(출력 이터레이터에서)로 변환함

```rust
fn map<B, F>(self, f: F) -> Map<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
```

클로저를 받아 각 요소에 대해 해당 클로저를 호출하는 이터레이터를 생성함

`map()` 은 인자(즉, `FnMut` 을 구현하는 무언가)를 통해 한 이터레이터를 다른 이터레이터로 변환함 원본 이터레이터의 각 요소에 대해 이 클로저를 호출하는 새로운 이터레이터를 생성함

## 예시

아래 코드에서 `map` 메소드는 입력 이터레이터의 각 `i32` 요소를 출력 이터레이터의 `String` 으로 변환함

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Transforming each number into its string representation
    // 각 숫자를 문자열 표현으로 변환
    let number_strings: Vec<String> = numbers.iter().map(|&num| num.to_string()).collect(); // 1, 2, 3, 4 설명에 해당

    println!("{:?}", number_strings);
}
```

1. `iter()` 는 `numbers` 에 대한 이터레이터를 생성함
2. 이 이터레이터에 `map()` 을 적용함
3. 클로저 `|&num| num.to_string()` 는 각 정수를 문자열 표현으로 변환하는 데 사용됨
4. `collect()` 메소드는 map 연산의 결과를 새로운 벡터 (`number_strings`)로 모으는 데 사용되며, 이 벡터는 숫자들의 문자열 표현을 포함함

## `map()` 은 변환을 위한 것이지, 부수 효과(side effects)를 위한 것이 아님

프로그래밍에서의 부수 효과란 함수나 연산이 자기 외부의 무언가를 변경하는 경우를 말함 예를 들어 전역 변수 수정, 콘솔 출력, 또는 단순히 값을 반환하는 대신 컬렉션이나 객체의 요소를 직접 변경하는 것과 같은 데이터의 제자리 수정 등이 있음


```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.iter_mut().for_each(|num| *num += 1);

    println!("{:?}", numbers);
}
```

```bash
[2, 3, 4, 5, 6]
```

`map` 을 사용하면, 제공된 클로저를 각 요소에 적용하는 새로운 이터레이터를 생성하지만, 이 적용은 지연(lazy) 평가됨 이는 클로저가 요소들에 즉시 적용되지 않고, 대신 이터레이터를 소비할 때 (일반적으로 반복문을 통하거나 `collect()` 와 같은 메소드를 사용하여) 적용됨을 의미함

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    let ret: Vec<_> = numbers.iter_mut().map(|num| *num += 1).collect();

    println!("{:?}", ret);
}
```

```bash
[(), (), (), (), ()]
```

클로저 ``|num| *num += 1`` 가 ``()`` 를 반환하기 때문에, `map` 은 각 요소가 ``()`` 인 이터레이터를 생성함

``()`` 는 "유닛 타입(unit type)"을 나타내며, 의미 있는 데이터가 없음을 의미함


```rust
// mapping and collecting elements into a Vec<T>, where T is a custom type
// 사용자 정의 타입 T에 대해 Vec<T>로 요소를 매핑하고 수집하기

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let data = [(1, 2), (3, 4), (5, 6)]; // 배열 또는 슬라이스

    let points: Vec<Point> = data.into_iter() // 배열/슬라이스에 into_iter() 호출 시 값/참조 반환
        .map(|(x, y)| Point{x, y}) // 튜플을 Point 구조체로 변환
        .collect();

    println!("{:?}", points);
}
```

```bash
Compiling playground v0.0.1 (/playground)
warning: fields `x` and `y` are never read
 --> src/main.rs:7:5
  |
6 | struct Point {
  |        ----- fields in this struct
7 |     x: i32,
  |     ^
8 |     y: i32,
  |     ^
  |
  = note: `Point` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: `playground` (bin "playground") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 3.59s
     Running `target/debug/playground`

[Point { x: 1, y: 2 }, Point { x: 3, y: 4 }, Point { x: 5, y: 6 }]
```