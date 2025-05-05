# Loops

- `loop`
    - 루프 내에서 **break** 문을 만날 때까지 무한정 계속되는 무한 루프를 만드는 데 사용됩니다
- `while`:
    - 지정된 조건이 `true`로 유지되는 한 코드 블록을 반복하는 데 사용됩니다
- `while let`
    - 이는 패턴과 일치하고 패턴에서 변수를 추출하여 패턴이 일치하는 한 계속 반복할 수 있는 while 루프의 변형입니다
- `for in`
    - 배열, 벡터 또는 범위와 같은 항목 컬렉션을 반복하는 데 사용됩니다. 다른 프로그래밍 언어의 for 루프와 유사합니다


### loop

레이블을 사용하여 루프를 식별한 다음 레이블 이름과 함께 `break` 또는 `continue` 문을 사용하여 특정 루프를 중단하거나 계속할 수 있습니다

```
'label_name: loop {
    // loop body
}
```

### loop with break

`break`가 실행되면 루프가 즉시 종료되고 프로그램은 루프 뒤의 명령문부터 계속 실행됩니다

```rust
fn main() {
    let mut i = 0;

    loop {
        if i == 3 {
            break;
        }
        println!("i = {}", i);
        i += 1;
    }

    println!("loop ends");
}
```

### break with return value

```rust
fn main() {
  let mut i = 0;
  
  let result = loop {
    if i == 3 {
      break i * 2;
    }
    i += 1;
  };
  
  println!("result = {}", result);
}
```

```rust
fn find_index_of_first_even_number(numbers: &[i32]) -> Option<usize> {
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            break None;
        }
        if numbers[index] % 2 == 0 {
            break Some(index);
        }
        index += 1;
    }
}
```

### break with label and return value

```rust
fn main() {
    'outer: loop {
        println!("Outer loop");

        'inner: loop {
            println!("Inner loop-1");

            loop {
                println!("Inner loop-2");
                break 'outer;
            }
        }
    }
    println!("Exited outer loop");
}
```

```rust
fn main() {
    let result = 'outer: loop {
        println!("Outer loop");

        'inner: loop {
            println!("Inner loop-1");

            loop {
                println!("Inner loop-2");
                break 'outer 20;
            } // inner-2 loop ends
        } // inner-1 loop ends
    }; // outer loop ends
    println!("Exited outer loop with result = {}", result);
}
```

## for in

```
for element in collection {
    // 컬렉션의 각 요소에 대해 실행할 코드
}
```

- 'element' 는 루프가 반복될 때마다 컬렉션의 각 요소에 할당되는 변수입니다. 루프 내부의 코드는 컬렉션의 각 요소에 대해 한 번씩 실행됩니다
- 컬렉션은 array, vector, range 또는 Iterator trait 을 구현하는 모든 유형일 수 있습니다


### for loop with a range of numbers

```rust
fn main() {
    'outer: for i in 0.. { 
        println!("{}", i);
        if i == 10 {
          break 'outer;
        }
    }
}

fn main() {
    for i in 2..=5 {
        println!("{}", i);
    }
}
```

- `i`는 루프 변수입니다. 범위는 선언된 루프 블록으로 제한됩니다
- `i`는 범위의 값과 동일한 유형입니다
- 값이 있는 `break`는 for 루프와 함께 사용할 수 없습니다