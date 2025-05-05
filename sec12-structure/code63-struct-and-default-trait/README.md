
## `#[derive(Default)]`

Rust 에서 구조체 유형에는 **Default** 특성에 대한 기본 구현이 없습니다  
구조체의 각 필드가 **Default** 특성을 구현하는 경우 `#[derive(Default)]` 속성 주석을 사용하여 자동으로 구조에 대한 기본 특성을 쉽게 구현할 수 있습니다

```rust
#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Default,Debug)]
struct Person {
    name: String,
    age: u8,
    height: f32,
    is_male: bool,
    initial: char,
}

fn main() {
    let person = Person::default();
    
    println!("Person: {:?}", person);
}

// Output:
// Person { name: "", age: 0, is_male: false, height: 0.0, initial: '\0' }
```


### 다른 구조체를 사용하여 구조체 업데이트

구조체 업데이트 구문에서 소스 및 대상 구조체는 동일한 유형이어야 합니다

```rust
#[derive(Debug)]
struct Process {
    name: String,
    pid: u32,
    group: String,
}

fn main() {
    let process1 = Process {
        name: String::from("Ping"),
        pid: 0x1234,
        group: String::from("Networking"),
    };
    println!("Process 1: {:#X?}", process1);

    // 이름 필드를 업데이트하여 새 프로세스를 만듭니다
    let process2 = Process {
        name: String::from("Route"),
        ..process1 // Struct literal update syntax
    };
    println!("Process 2: {:#X?}", process2);

    // pid 및 group 필드를 업데이트하여 새 프로세스를 만듭니다
    let process3 = Process {
        pid: 0x3456,
        group: String::from("Security"),
        ..process2
    };
    println!("Process 3: {:#X?}", process3);
}
```

코드에 표시된 대로 이 출력 `use:#X?` 형식 지정자를 얻으려면

```shell
Process 1: Process {
    name: "Ping",
    pid: 0x1234,
    group: "Networking",
}

Process 2: Process {
    name: "Route",
    pid: 0x1234,
    group: "Networking",
}

Process 3: Process {
    name: "Route",
    pid: 0x3456,
    group: "Security",
}
```