
## 변수를 사용하여 구조체의 인스턴스 만들기

```rust
fn main() {
    let tmp_name = String::from("Alice");
    let tmp_age = 25;
    // creating an instance of the struct Person
    let person = Person {
        name: tmp_name,
        age: tmp_age,
        address: String::from("123 Main St"),
    };
}
```

필드 초기화 단축 구문을 사용하면 구조체의 필드와 동일한 이름을 가진 변수를 사용하여 구조체의 새 인스턴스를 만들 수 있습니다

```rust
fn main() {
    let name = String::from("Alice");
    let age = 25;
    let address = String::from("Main St");

    // creating an instance of Person
    let person = Person {
        name,
        age,
        //address, // <- This line is commented out
    };

    struct Point {
        x: i32,
        y: i32,
    }

    fn create_point(x1: i32, y1: i32) -> Point {
        Point { x: x1, y: y1 }
    }

    fn create_point(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}
```


## `#[derive(Debug)]`

- `#[derive(Debug)]` 주석은 Rust 컴파일러에게 구조체에 대한 **Debug** 특성을 구현하는 코드를 자동으로 생성하도록 지시합니다
- `{:?}` 형식 지정자를 사용하여 디버깅 목적으로 구조체를 인쇄할 수 있습니다
- Rust의 **derive** 속성을 사용하면 개발자가 데이터 구조의 다양한 특성에 대한 자동 구현을 쉽게 만들 수 있습니다  
  이러한 특성에는 **Debug**, **Clone**, **Copy**, **PartialEq** 등이 포함될 수 있습니다  
  특성의 이름과 함께 **derive** 속성을 데이터 구조 정의에 추가함으로써 Rust는 해당 데이터 구조에 대한 해당 특성의 구현을 자동으로 생성합니다.


## Mutable struct

`mut` 키워드를 사용하여 구조체 변수를 선언하면 모든 필드를 포함하여 전체 변수를 변경 가능하게 만듭니다

```rust
# [derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

fn main() {
    // create a new mutable Person struct
    let mut person = Person {
        name: String::from("Alice"),
        age: 25,
        address: String::from("123 Main St"),
    };

    // update the name field
    person.name = String::from("Bob");

    // print the updated struct
    println!("{:?}", person);

    // move. 'name' field of struct is uninitialized
    let name = person.name;
    println!("{}", person.age); // OK
    println!("{:?}", person); // Error
}
```

### By default, variable bindings have 'move semantics'

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 is moved to p2
    println!("p1: {:?}, p2: {:?}", p1, p2); // Error
}
```

### Is struct `copy` or `move`?

- 구조체가 `Copy`인지 `Move`인지는 해당 멤버 필드의 유형에 따라 다릅니다
- 구조체 변수를 다른 변수에 할당하면 구조체가 `Copy` 특성을 구현하지 않기 때문에 기본적으로 `Move`입니다




```rust
// 'Clone' 은 'Copy' 의 상위 특성이므로 Copy 인 모든 항목도 Clone 을 구현해야 합니다
// String 과 같이 Copy 가 아닌 필드를 포함하는 유형에 대해서는 Copy 특성을 구현할 수 없습니다
#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 is copied to p2
    let p3 = p1.clone(); // p1 is cloned to p3
    println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3); // OK
}
```

이 특별한 경우에는 `clone()`과 `copy()` 모두 동일한 결과를 생성하여 구조체에 있는 데이터의 비트 단위 복사본을 만듭니다  
따라서 `p1`의 값은 `p2`에 복사되고 `p1`은 `p3`에 복사됩니다  

하지만 항상 그런 것은 아니며 어떤 상황에서는 `Clone`과 `Copy`의 동작이 다를 수 있습니다

https://doc.rust-lang.org/stable/std/marker/trait.Copy.html