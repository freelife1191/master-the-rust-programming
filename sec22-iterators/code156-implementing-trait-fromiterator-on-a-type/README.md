
## Trait `std::iter::FromIterator`

* 이 Trait은 iterator로부터의 변환에 사용됨 어떤 타입에 대해 이 Trait을 구현할 때, 해당 타입이 iterator로부터 어떻게 생성될지를 정의함
* 이 Trait은 iterator의 `collect()` method를 가능하게 하는 메커니즘임 iterator에 대해 `collect()` 를 호출하면, Rust는 iterator의 요소들로부터 해당 타입을 생성하기 위해 대상 타입의 `FromIterator` 구현을 사용함 이 Trait 덕분에 `collect()` 가 극도로 유연하고 강력해짐

```rust
pub trait FromIterator<A>: Sized {
    // Required method
    // 필요한 메소드
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}
```

* **`from_iter()`**: iterator로부터 해당 타입의 인스턴스를 생성하는 방법을 정의함
* **`: Sized`**: `FromIterator` 를 구현하는 모든 타입은 반드시 `Sized` Trait도 만족해야 함을 명시함 즉, 해당 타입은 컴파일 타임에 알려진 크기를 가져야 함
* **`A`**: iterator가 생성할 아이템의 타입임 어떤 타입에 대해 `FromIterator<A>` 를 구현할 때, 타입 A의 아이템을 생성하는 iterator로부터 해당 타입의 인스턴스를 구성하는 방법을 명시하는 것임 예를 들어, 사용자 정의 타입에 대해 `FromIterator<i32>` 를 구현하면, `i32` 값의 iterator로부터 수집할 때 해당 구현이 사용됨
* **`T`**: iterator 자체의 타입을 나타냄 구체적으로, `T` 는 `IntoIterator<Item = A>` 를 구현하는 타입임 이는 `T` 가 타입 `A` 의 아이템을 생성하는 iterator로 변환될 수 있는 모든 타입이 될 수 있음을 의미함
* **`Self`**: iterator를 변환하려는 대상 타입임 특정 타입에 대해 `FromIterator` Trait을 구현할 때, `Self` 는 해당 타입으로 대체됨

## 아이템 타입 호환성 참고사항

* `iterator` 의 아이템 타입은 `FromIterator` 구현이 기대하는 것과 호환되어야 함
* 예를 들어, 사용자 정의 타입에 대해 `FromIterator<i32>` 를 구현했다면, `i32` 아이템을 생성하는 `iterator` 에 대해서만 `collect()` 를 사용할 수 있음 `String` 이나 `f64` 와 같이 다른 타입을 생성하는 `iterator` 에 대해서는 해당 타입들에 대한 추가 구현 없이는 이 구현을 사용할 수 없음



```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Transaction {
    amount: f64,
    description: String,
}

impl FromIterator<Transaction> for (f64, usize) {
    fn from_iter<T>(iter: T) -> Self
    where T: IntoIterator<Item = Transaction>
    {
        let mut total_amount = 0.0;
        let mut total_transcations = 0; // 변수명 오타 가능성 (transactions)

        for transcation in iter { // 변수명 오타 가능성 (transaction)
            total_amount += transcation.amount;
            total_transcations += 1;
        }

        (total_amount, total_transcations)
    }
}

fn main() {
    let transactions = vec![
        Transaction {
            amount: 100.0,
            description: "Groceries".to_string(),
        },
        Transaction {
            amount: 40.0,
            description: "Gas".to_string(),
        },
        Transaction {
            amount: 60.0,
            description: "Internet Bill".to_string(),
        },
    ];

    let transaction_map: HashMap<String, f64> = transactions.into_iter().collect();

    println!("{:?}", transaction_map);
}
```

```bash
Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0277]: a value of type `HashMap<String, f64>` cannot be built from an iterator over elements of type `Transaction`
  --> src/main.rs:58:74
   |
58 |     let transcation_map: HashMap<String, f64> = transactions.into_iter().collect();
   |                                                                          ^^^^^^^ value of type `HashMap<String, f64>` cannot be built from `std::iter::Iterator<Item=Transaction>`
   |
   = help: the trait `FromIterator<Transaction>` is not implemented for `HashMap<String, f64>`
   = help: the trait `FromIterator<(String, f64)>` is implemented for `HashMap<String, f64>` // Note: Suggests collecting from tuples, not Transactions directly
   = help: for that trait implementation, expected `(String, f64)`, found `Transaction`
note: the method call chain might not have had the expected associated types
  --> src/main.rs:58:62
   |
43 |     let transactions = vec![
   |                        -----
...
46 |             description: "Groceries".to_string(),
   |             -----------------------------------
...
55 |         },
56 |     ];
   |     -- this expression has type `Vec<Transaction>`
   ...
58 |     let transcation_map: HashMap<String, f64> = transactions.into_iter().collect();
   |                                                              ^^^^^^^^^^
```

```rust
use std::collections::HashMap;

fn main() {
    let numbers = vec![(1, 2), (3, 4)];
    let hashmap: HashMap<_, _> = numbers.into_iter().collect();
    println!("{:?}", hashmap);
}
```

```bash
{1: 2, 3: 4}
```