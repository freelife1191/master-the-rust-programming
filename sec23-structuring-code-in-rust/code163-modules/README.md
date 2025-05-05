# Modules

* Rust는 `project` 내에서 `code` 의 프라이버시(privacy)와 네임스페이스(namespace)를 구성(organising), 재사용(reusing), 제어(controlling)하는 데 사용되는 `module` 시스템을 사용함
* `crate` 는 계층적(hierarchical) `module` `structure` 를 포함할 수 있으며, 이를 통해 더 나은 구성(organisation), 캡슐화(encapsulation), 명확성(clarity)을 위해 `code` 를 다른 네임스페이스(namespaces)로 구성하고 구조화할 수 있음

> Rust의 `module` 은 관련된 `functions`, `structs`, `Traits`, 상수(constants), 그리고 다른 `module` 들을 그룹화하기 위한 컨테이너(container)로, `program` 내에서 `code` 를 구성하고 가시성(visibility)을 제어하는 방법을 제공함

```bash
# crate
crate (main.rs or lib.rs) # Crate root
|-- mod module1 {         # Module
|   |-- fn function1()    # Function
|   |-- struct Struct1    # Struct
|   `-- ...               # Other constructs
|-- mod module2 {         # Another module
|   `-- mod submodule {   # Sub-module
|       `-- ...
`-- ...
```

## Why modules?

`project` 에서 `module` 을 사용하는 주요 이유

**Hierarchical Structure (계층적 구조)**

Rust의 `module` 시스템은 `crate` 내부에 트리(tree) 같은 `module` 계층(hierarchy)을 생성할 수 있게 함 이 계층은 `code` 내의 논리적 분할(logical divisions)을 반영하여, 다른 기능(functionalities), 컴포넌트(components), 또는 도메인(domains)을 분리할 수 있음

```bash
math_library_crate
|-- mod arithmetic
|   |-- mod operations
|   |   |-- fn add
|   |   `-- fn subtract
`-- mod geometry
    `-- mod shapes
        |-- mod rectangle
        |   `-- fn area
        `-- mod circle
            `-- fn area
```

## `project` 에서 `module` 을 사용하는 주요 이유

* **Namespaces**
    * Rust의 각 `module` 은 자체 `namespace` 로 작동함 이는 다른 `module` 에서 충돌 없이 동일한 이름의 `item` (예: `functions`, `structs`, `enums` 등)을 가질 수 있음을 의미함
* **Encapsulation and privacy (캡슐화 및 프라이버시)**
    * Rust는 `module` 수준에서 프라이버시(privacy) 경계를 강제하므로, `module` 은 `code` 를 캡슐화(encapsulate)하는 데 사용될 수 있음 기본적으로 `module` 내의 `item` (`functions`, `structs` 등)은 `private` 이며, 명시적으로 `public` (`pub`)으로 선언되지 않는 한 해당 `module` 내에서만 접근 가능함
* **Organization and clarity (구성 및 명확성)**
    * `Module` 은 더 쉬운 `navigation`, `understanding`, `maintenance` 를 위해 `code` 를 구성하는 데 도움이 됨
* **Reusability (재사용성)**
    * `Module` 은 `code` `reusability` 를 가능하게 하여, `functionalities` 를 한 번 정의하고 여러 곳에서 사용함으로써 중복성(redundancy)을 줄일 수 있게 함

## Keywords

Rust에서 `module` 로 작업할 때 다음 `keyword` 들을 마주칠 수 있음:

1.  **`mod`**: 이 `keyword` 는 새 `module` 을 정의함 `code` 를 논리적 단위(logical units)로 구성하는 데 사용되며, 파일 기반(file-based) `module` 과 인라인(inline) `module` 모두를 생성하는 데 사용될 수 있음
2.  **`use`**: 이 `keyword` 는 `module`, `type`, `function`, 또는 다른 `item` 들을 현재 스코프(scope)로 가져와서, 전체 경로(full `path`) 명시 없이 접근할 수 있게 함
3.  **`pub`**: 이 `keyword` 는 `item` 을 `public` 상태로 만듦 기본적으로, `module` 내용물 (`functions`, `structs`, `enums` 등)은 `private` 이며 해당 `module` 또는 자식 `module` 내에서만 접근 가능함 `pub` 은 이러한 `item` 들이 `module` 외부에서도 접근 가능하도록 허용함

```rust
mod cars {
    pub struct Car { // main에서 사용하려면 pub 필요
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(car: &Car) { // 예시 함수 (사용되지 않음)
        println!("Car Make: {}, Model: {}, Year: {}", car.make, car.model, car.year);
    }
}

mod motorcycles {
    pub struct Motorcycle { // 예시 구조체 (사용되지 않음)
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(motorcycle: &Motorcycle) { // 예시 함수 (사용되지 않음)
        println!("Motorcycle Make: {}, Model: {}, Year: {}", motorcycle.make, motorcycle.model, motorcycle.year);
    }
}

fn main() {
    let my_car = cars::Car { // cars 모듈의 Car 구조체 인스턴스 생성
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2020,
    };

    let my_motorcycle = motorcycles::Motorcycle { // motorcycles 모듈의 Motorcycle 구조체 인스턴스 생성
        make: "Honda".to_string(),
        model: "CBR".to_string(),
        year: 2018,
    };

    cars::info(&my_car); // cars 모듈의 info 함수 호출
    motorcycles::info(&my_motorcycle); // motorcycles 모듈의 info 함수 호출
}
```

```bash
Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0603]: struct `Car` is private
  --> src/main.rs:26:24
   |
26 |     let my_car = cars::Car {
   |                        ^^^ private struct
   |
note: the struct `Car` is defined here
  --> src/main.rs:2:6
   |
2  | struct Car {
   |      ^^^^

error[E0603]: struct `Motorcycle` is private
  --> src/main.rs:32:38
   |
32 |     let my_motorcycle = motorcycles::Motorcycle {
   |                                      ^^^^^^^^^^ private struct
   |
note: the struct `Motorcycle` is defined here
  --> src/main.rs:14:6
   |
14 | struct Motorcycle {
   |      ^^^^^^^^^^

error[E0603]: function `info` is private
  --> src/main.rs:38:11
   |
38 |     cars::info(&my_car);
   |           ^^^^ private function
   |
note: the function `info` is defined here
  --> src/main.rs:8:6
   |
8  | fn info(car: &Car) {
   |      ^^^^

error[E0603]: function `info` is private
  --> src/main.rs:39:18
   |
39 |     motorcycles::info(&my_motorcycle);
   |                  ^^^^ private function
   |
note: the function `info` is defined here
  --> src/main.rs:20:6
   |
20 | fn info(motorcycle: &Motorcycle) {
   |      ^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `playground` (bin "playground") due to 4 previous errors
```

Rust에서 `module` 로 작업할 때 다음 `keyword` 들을 마주칠 수 있음:

4.  **`crate`**: `module` 의 맥락에서, `crate` 는 현재 `crate` (최상위 `package` 또는 `library`)를 참조함 `crate` 의 루트(root)에 있는 `item` 을 참조하는 데 사용됨
5.  **`super`**: 이 `keyword` 는 현재 `module` 의 부모 `module` 을 참조함 현재 `module` 계층(hierarchy)에서 한 단계 위에 있는 `module` 의 `item` 에 접근하는 데 유용함
6.  **`self`**: 이 `keyword` 는 현재 `module` 자체를 참조함 종종 `use` 구문에서 동일한 `module` 내의 `item` 을 참조하는 데 사용됨

```bash
+-----------------------------------------+
| implicit top-level module               |
| (crate)                                 |
|                                         |
| +--------+      +-------------+         |
| | cars   |      | motorcycles |         |
| +--------+      +-------------+         |
|                                         |
|      main() function                    |
+-----------------------------------------+
```

Top-level anonymous `module` 이며, `crate::` `prefix` 를 사용하여 해당 `item` 들을 참조할 수 있음

```rust
mod cars {
    pub struct Car {
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(car: &Car) {
        println!("Car Make: {}, Model: {}, Year: {}", car.make, car.model, car.year);
    }
}

mod motorcycles {
    pub struct Motorcycle {
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(motorcycle: &Motorcycle) {
       println!("Motorcycle Make: {}, Model: {}, Year: {}", motorcycle.make, motorcycle.model, motorcycle.year);
    }
}


fn main() {
    let my_car = cars::Car { // cars 모듈의 Car 구조체 인스턴스 생성
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2020,
    };

    let my_motorcycle = motorcycles::Motorcycle { // motorcycles 모듈의 Motorcycle 구조체 인스턴스 생성
        make: "Honda".to_string(),
        model: "CBR".to_string(),
        year: 2018,
    };

    cars::info(&my_car); // cars 모듈의 info 함수 호출
    motorcycles::info(&my_motorcycle); // motorcycles 모듈의 info 함수 호출
}
```

```bash
Car Make: Toyota, Model: Camry, Year: 2020
Motorcycle Make: Honda, Model: CBR, Year: 2018
```

```rust
fn log_vehicle_details(make: &str, model: &str, year: u32) {
    println!("vehicle Make: {}, Model: {}, Year: {}", make, model, year);
}

mod cars {
    // (mod cars { 선언이 이 코드 위에 있을 것으로 예상됨)
    pub struct Car {
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(car: &Car) {
        crate::log_vehicle_details(&car.make, &car.model, car.year); // crate::log_vehicle_details 함수 정의 필요
        //println!("Car Make: {}, Model: {}, Year: {}", car.make, car.model, car.year);
    }
}

mod motorcycles {
    pub struct Motorcycle {
        pub make: String,
        pub model: String,
        pub year: u32,
    }

    pub fn info(motorcycle: &Motorcycle) {
        crate::log_vehicle_details(&motorcycle.make, &motorcycle.model, motorcycle.year); // crate::log_vehicle_details 함수 정의 필요
        //println!("Motorcycle Make: {}, Model: {}, Year: {}", motorcycle.make, motorcycle.model, motorcycle.year);
    }
}

fn main() {
    let my_car = cars::Car { // cars 모듈의 Car 구조체 인스턴스 생성
        make: "Toyota".to_string(),
        model: "Camry".to_string(),
        year: 2020,
    };

    let my_motorcycle = motorcycles::Motorcycle { // motorcycles 모듈의 Motorcycle 구조체 인스턴스 생성
        make: "Honda".to_string(),
        model: "CBR".to_string(),
        year: 2018,
    };

    cars::info(&my_car); // cars 모듈의 info 함수 호출
    motorcycles::info(&my_motorcycle); // motorcycles 모듈의 info 함수 호출
}
```

```rust
mod parent {
    fn parent_private_function() {
        println!("I'm private to the parent module");
    }

    pub fn parent_public_function() {
        println!("I'm public in the parent module");
    }

    pub fn access_child_private_function() {
        // child::child_private_function(); // 오류: child_private_function은 private임
    }

    pub mod child {
        pub fn child_public_function() {
            // call private function of parent
            // 부모 모듈의 private 함수 호출
            super::parent_private_function(); // OK: 자식은 부모의 private 항목에 접근 가능 (특정 조건 하)
        }

        fn child_private_function() {
            println!("I'm child fn and not public");
        }
    }
}

fn main() {
    parent::child::child_public_function();
    // parent::access_child_private_function();
}
```

```bash
I'm private to the parent module
```

`child::child_private_function()` 호출시 에러 메세지

```bash
Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0603]: function `child_private_function` is private
  --> src/main.rs:13:16
   |
13 |         child::child_private_function();
   |                ^^^^^^^^^^^^^^^^^^^^^^^^ private function
   |
note: the function `child_private_function` is defined here
  --> src/main.rs:22:9
   |
22 |     fn child_private_function() {
   |         ^^^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```