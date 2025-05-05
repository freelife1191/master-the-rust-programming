# Tuples

- 튜플은 서로 다른 유형의 여러 값을 하나의 순서가 지정된 컬렉션으로 그룹화할 수 있는 데이터 구조입니다

```
let my_tuple = (1, "hello", true);
let my_tuple: (i32, &str, bool) = (1, "hello", true);
```

- `my_tuple` 튜플의 유형은 `(i32, &str, bool)`입니다
- Rust 의 튜플에는 명명된 매개변수가 없습니다. 대신, 튜플의 요소는 0부터 시작하는 인덱싱을 사용하여 위치에 따라 액세스됩니다


```rust
fn main() {
    let my_tuple: (i32, &str, bool) = (1, "hello", true);
    println!("{}", my_tuple);
}
```

## Accessing tuple elements

> 점 표기법과 액세스하려는 요소의 인덱스를 사용하여 튜플의 요소에 액세스할 수 있습니다.

```
let my_tuple = (1, "hello", true);

let my_int = my_tuple.0;
let my_string = my_tuple.1;
let my_bool = my_tuple.2;
```

```
// Tuple destructuring
let (my_int, my_string, my_bool) = my_tuple;
```

튜플 인덱스 번호는 가변적일 수 없습니다  
튜플 인덱스는 컴파일 타임에 고정되며 튜플의 요소에 액세스하려면 상수 정수 리터럴을 사용해야 합니다  
변수나 표현식을 튜플의 인덱스 값으로 사용할 수 없습니다

```rust
fn main() {
    //A tuple is a heterogeneous data type, that means it can hold elements of different types.
    let my_tuple = (1, "hello", true);

    //elements of a tuple are accessed using index numbers
    let my_int = my_tuple.0;
    let my_string = my_tuple.1;
    let my_bool = my_tuple.2;

    //or

    //Tuple destructuring
    let (my_int, my_string, my_bool) = my_tuple;

    //to print tuple use {:?} format specifier because Tuples dont implement the Display trait
    println!("{:?}", my_tuple);
}
```

## Return multiple values from a function

```rust
fn my_function() -> (i32, String, bool) {
    let my_int = 42;
    let my_string = "hello".to_string();
    let my_bool = true;
    (my_int, my_string, my_bool)
}

fn main() {
    let result = my_function();
    println!("{:?}", result);

    // Destructuring the tuple
    let (value, msg, is_ok) = result;
    println!("{}", is_ok);
}
```

## Passing tuple by reference

컴파일러는 튜플 또는 구조체에 대한 참조를 자동으로 역참조합니다
이 경우에는 역참조 연산자(*)를 명시적으로 사용할 필요가 없습니다

```rust
fn increment_elements(tuple: &mut (i32, i32, i32)) {
    tuple.0 += 1;
    tuple.1 += 1;
    tuple.2 += 1;
}

fn main() {
    let mut the_tuple = (1, 2, 3);
    increment_elements(&mut the_tuple);
    println!("{:?}", the_tuple);
}
```

Rust 는 튜플이나 구조체나 열거형의 객체와 같은 객체에 대한 참조가 있을 때 메서드 호출과 필드 액세스에 대한 자동 역참조를 제공합니다

## Tuples can be nested

```rust
fn main() {
    let grid = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    println!("Middle element: {}", grid.1.1);
}
```

튜플은 다른 튜플 안에 중첩되어 더 복잡한 데이터 구조를 만들 수 있습니다
예를 들어, 튜플의 튜플을 사용하여 3x3 숫자 격자를 나타낼 수 있습니다
