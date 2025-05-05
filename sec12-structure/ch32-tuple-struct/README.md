# Tuple struct

튜플 구조체는 명명된 필드가 없다는 점에서 튜플과 유사합니다  
대신, 튜플 내의 인덱스를 통해 필드에 액세스합니다  
그러나 일반 튜플과 달리 튜플 구조체에는 이름이 있으므로 코드에 추가 컨텍스트와 명확성을 제공하는 데 도움이 될 수 있습니다


```rust
struct Point(i32, f64, u8);
fn main() {
    let point = Point(10, 3.5, 1);
    println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);
}
```

두 튜플 구조체의 필드 유형 순서가 동일한 경우 필드 유형의 이름과 값이 동일하더라도 서로 다른 유형으로 간주됩니다  
따라서 필드 유형이 동일한 경우 하나의 튜플 구조체를 다른 튜플 구조체로 전달할 수 없습니다


```rust
struct Size(i32, i32, i32);
struct Point(i32, i32, i32);

fn refactor_point(point: &mut Point) {
    point.0 += 5;
    point.1 += 10;
}

fn print_point(point: Point) {
    println!("x: {}, y: {}, z: {}", point.0, point.1, point.2);
}

fn main() {
    let size = Size(0, 0, 0);
    let mut origin = Point(0, 0, 0);

    refactor_point(&mut origin);
    print_point(origin);

    // tuple struct를 사용하여 데이터 수집에 대한 유형 안전성을 달성할 수 있습니다
    print_point(size); //Error
}
```

## Summary

튜플 구조체는 전체 튜플에 이름을 지정하고 다른 튜플과 구별되는 유형으로 만들려고 할 때 사용되지만 일반 구조체에서 각 필드의 이름을 지정하는 것은 장황하거나 중복됩니다  
명명된 필드의 오버헤드 없이 가벼운 구조체형 데이터 구조가 필요한 경우에도 사용할 수 있습니다


## A Unit type struct

크기가 0인 구조체라고도 알려진 단위 유형 구조체는 필드나 크기가 없는 Rust의 구조체입니다  
필드 없이 `struct Name;`으로 선언됩니다  
유형을 표현해야 하지만 마커 특성이나 상태 표시와 같은 데이터를 저장할 필요가 없는 상황에서 유용합니다

1. 추가 데이터 없이 유형에 대한 정보를 전달하는 마커 특성을 구현하는 데 사용할 수 있습니다
2. As Enum Variants: 추가 데이터가 필요하지 않은 경우를 나타내기 위해 enum 내의 변형으로 사용할 수 있습니다
3. 모듈 수준 상수의 자리 표시자


```rust
#[derive(Debug)]
struct NoData;

// 단위 유형 대신 반환 유형으로 구조체를 사용하면 반환 값에 이름을 지정할 수 있어 코드가 더 자체적으로 문서화되고 이해하기 쉬워집니다
fn do_something() -> NoData {
    // do something that doesn't return any data
    NoData
}

fn main() {
    let _ = do_something();
    println!("{:?}", result); // prints "NoData"
}
```