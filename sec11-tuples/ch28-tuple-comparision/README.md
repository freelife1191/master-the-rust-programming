## Tuples comparison

튜플의 모든 유형이 `PartialOrd` 및 `PartialEq` trait 를 구현하는 한 `==`, `!=`, `<`, `<=`, `>` 및 `>=`와 같은 연산자를 사용하여 튜플을 사전식으로 비교할 수도 있습니다

`PartialOrd` trait 은 두 값 사이의 순서 관계를 정의하는 데 사용되는 반면, **PartialEq** trait 은 두 값 사이의 동등 관계를 정의하는 데 사용됩니다

```rust
fn main() {
    let tuple1 = (2, 3, 4);
    let tuple2 = (8, 3, 4);
    let tuple3 = (1, 2, 3);

    // 가능한 경우: 모든 요소가 비교 가능함
    if tuple1 < tuple2 {
        println!("tuple1 is smaller");
    } else {
        println!("tuple2 is smaller")
    }

    if tuple1 == tuple3 {
        println!("Tuples equal")
    } else {
        println!("Tuples not equal")
    }

    let tuple4 = (1, "world x");
    let tuple5 = (1, "world z");
    let tuple6 = (1, "world x");

    if tuple4 > tuple5 {
        println!("tuple4 is bigger")
    } else {
        println!("tuple5 is bigger")
    }

    if tuple4 == tuple6 {
        println!("Tuples equal")
    } else {
        println!("Tuples not equal")
    }
}
```

## Pattern matching using tuple

첫 번째 요소가 0보다 크고 마지막 요소가 10보다 작은 경우에만 중간 요소를 처리합니다

```rust
fn main () {
    let rovd_data = (5, "hello", 8) ;
    match revd_data {
        (a, s, c) if a > 0 && c < 10 => {
            println! ("Valid data: s = {}", s) ;
        }
        _ => println! ("Invalid data") ,
    }
}
```

