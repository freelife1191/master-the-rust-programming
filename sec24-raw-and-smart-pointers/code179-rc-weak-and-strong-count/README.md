# `Rc<T>`: Strong and weak count

## Memory representation

![img1.png](assets/img1.png)

## Strong count

* **Strong count**와 **weak count**는 **shared data**의 **lifecycle**을 관리하기 위한 **internal bookkeeping**에 사용된다
* **strong count**가 0보다 큰 한, **underlying data**는 살아있음이 보장된다
* `Rc<T>`를 **clone**하면, 이는 또 다른 **strong reference**를 생성하여, **strong count**를 1 증가시킨다
* **strong count**가 0에 도달하면, **underlying data** (`T`)는 drop되고 (그것의 **Drop implementation**이 실행된다), 하지만 **weak references**가 있다면 **reference counts**를 포함하는 **allocation**은 여전히 남아있을 수 있다

```rust
use std::rc::Rc;

fn main() {
    let s1 = Rc::new(42);
    println!("{}", Rc::strong_count(&s1)); //prints 1

    {
        let s2 = Rc::clone(&s1);
        println!("{}", Rc::strong_count(&s1)); //prints 2
    }

    println!("{}", Rc::strong_count(&s1)); //prints 1
}
```

```rust
use std::rc::Rc;

fn main() {
    let strong_rc_1: Rc<String> = Rc::new(String::from("Hello, Rust!"));
    println!("{}", Rc::strong_count(&strong_rc_1));
    println!("{}", Rc::weak_count(&strong_rc_1));

    let strong_rc_2 = Rc::clone(&strong_rc_1);
    println!("{}", Rc::strong_count(&strong_rc_1));
    println!("{}", Rc::weak_count(&strong_rc_1));

    // weak reference를 생성하기
}
```

```bash
1
0
2
0
```

## Weak count

- `Weak<T>` **references**의 수는 **the same allocation**에 대한 것으로 **data**를 살아있게 유지하지 않는 것이다
- **Weak references**는 **data**의 **lifetime**에 영향을 주지 않으며 직접적으로 **dereference**될 수 없다
- **data**에 접근하기 위해 `Weak::upgrade()` (via `Weak::upgrade()`)를 통해 `Rc<T>` 로 **upgrade**되어야 한다 그리고 이 **upgrade**는 **data**가 여전히 살아있는 경우(즉, `strong count > 0`)에만 성공한다
- **weak references**를 생성하기 위해 `Rc::downgrade()` 를 사용한다 { `Rc::downgrade()` 는 `Weak<T>` 의 **instance**를 생성한다 }
- **strong count**가 감소하거나 **data**가 **drop**될 때 **weak count**에 영향을 주지 않는다

```rust
use std::rc::Rc;
use std::rc::Weak;

fn main() {
    let strong_rc_1: Rc<String> = Rc::new(String::from("Hello, Rust!"));
    println!("{}", Rc::strong_count(&strong_rc_1));
    println!("{}", Rc::weak_count(&strong_rc_1));

    let strong_rc_2 = Rc::clone(&strong_rc_1);
    println!("{}", Rc::strong_count(&strong_rc_1));
    println!("{}", Rc::weak_count(&strong_rc_1));

    // **weak reference**를 생성하기
    let weak_ref_1 = Rc::downgrade(&strong_rc_1);
    let weak_ref_2 = weak_ref_1.clone();
    println!("{}", Rc::strong_count(&strong_rc_1));
    println!("{}", Rc::weak_count(&strong_rc_1));

    // **weak ref**를 **strong ref**로 **upgrade**하기
    let strong_rc_3: Option<Rc<String>> = weak_ref_1.upgrade();
    println!("{}", Rc::strong_count(&strong_rc_1));
    println!("{}", Rc::weak_count(&strong_rc_1));

    // **data access**
    println!("{}", strong_rc_1);
    // println!("{}", weak_ref_1);

    if let Some(upgraded_rc) = weak_ref_1.upgrade() {
        println!("Data is alive {}", upgraded_rc); // Data is accessible via upgraded_rc
    } else {
        println!("Data is not alive");
    }

    // **Data**는 **strong_rc_1**와 **strong_rc_2**가 여기서 **scope**를 벗어날 때 **dropped**된다
}
```

```bash
1
0
2
0
2
2
3
2
Hello, Rust!
Data is alive Hello, Rust!
```

```rust
use std::rc::Rc;

fn main() {
    // `Rc<T>`의 data를 mutate하는 방법
    let mut strong_rc_1: Rc<i32> = Rc::new(5);
    let mut strong_rc_2 = strong_rc_1.clone();

    if let Some(r) = Rc::get_mut(&mut strong_rc_1) {
        *r = 10;
    }

    println!("{{}}", strong_rc_1);
}
```

```bash
5
```

## Summary: Mutating 'T' in Rc<T>

- **directly mutate**할 수 없다 **data**를 `Rc<T>` 안에서 왜냐하면 이것이 **shared ownership**과 **read-only access**를 보장하기 때문이다
- **mutation**을 가능하게 하려면 사용할 수 있다:
    - `RefCell<T>`: **single-threaded contexts**를 위한 **interior mutability**를 **Provides**한다
    - `Rc::get_mut()`: **direct mutation**을 **Allows**하지만 다른 **references**가 없는 경우에만 **works**