# Rust가 메모리 안전(memory safe)하다고 말하는 것은 무엇을 의미하는가? Part-3

## 두려움 없는 동시성 (Fearless concurrency)

* **안전한 Rust(`safe Rust`)** 에서는, `Mutex`나 `RwLock`과 같은 동기화 기본 요소(synchronization primitives)를 사용하지 않으면 데이터 경쟁(data race)이 발생하는 것이 불가능함 이는 Rust의 소유권 시스템과 빌림 검사기(borrow checker)가 공유 가변 상태(shared mutable state)에 대한 배타적 접근을 강제하기 때문임
* 데이터 경쟁을 유발하는 코드를 작성하려고 하면, Rust 컴파일러는 해당 코드의 컴파일을 막을 것임 안전하지 않은 방식으로 공유 가변 상태에 접근하려고 한다는 오류 메시지를 받게 될 것임
* `C++`에서는 적절한 동기화를 사용하더라도 데이터 경쟁이 발생할 수 있음

```rust
use std::thread;

fn main() {
    // Immutable data
    let data = 42;

    let handle1 = thread::spawn(|| {
        println!("Thread 1 reads data: {}", data);
    });

    let handle2 = thread::spawn(|| {
        println!("Thread 2 reads data: {}", data);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

```bash
Errors

Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0373]: closure may outlive the current function, but it borrows `data`, which is owned by the current function
 --> src/main.rs:8:33
  |
8 |     let handle1 = thread::spawn(|| {
  |                                 ^^ may outlive borrowed value `data`
9 |         println!("Thread 1 reads data: {}", data);
  |                                            ---- `data` is borrowed here
  |
note: function requires argument type to outlive `'static`
 --> src/main.rs:8:19
  |
8 |     let handle1 = thread::spawn(|| {
  |                   ^^^^^^^^^^^^^^^^^^
...
10|     });
  |     -
help: to force the closure to take ownership of `data` (and any other referenced variables), use the `move` keyword
  |
8 |     let handle1 = thread::spawn(move || {
  |                                 ++++
```

```rust
fn main() {
    let x: i32;
    println!("{}", x);
}
```

```bash
Errors

Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0381]: used binding `x` isn't initialized
 --> src/main.rs:3:20
  |
2 |     let x: i32;
  |     - binding declared here but left uninitialized
3 |     println!("{}", x);
  |                    ^ `x` used here but it isn't initialized
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider assigning a value
  |
2 |     let x: i32 = 42;
  |                +++++

For more information about this error, try `rustc --explain E0381`.
error: could not compile `playground` (bin "playground") due to 1 previous error

Standard Output
```

## 타입 혼란 (Type Confusion)

* 타입 혼란은 프로그램이 메모리 블록을 실제 타입과 다른 타입으로 잘못 해석할 때 발생함
* 이는 정의되지 않은 동작, 메모리 손상, 또는 심지어 보안 취약점으로 이어질 수 있음

```c
#include <stdio.h>

typedef struct {
    int x;
    int y;
} Point;

typedef struct {
    float radius;
    float angle;
} Circle;

// void* allows the function to accept any type of pointer without enforcing type safety
void print_shape(void* shape, int is_circle) {
    if (is_circle) {
        // Program blindly trusts the cast, even though the actual memory layout does not match
        Circle* circle = (Circle*)shape;
        printf("Circle: radius = %.2f, angle = %.2f\n", circle->radius, circle->angle);
    } else {
        Point* point = (Point*)shape;
        printf("Point: x = %d, y = %d\n", point->x, point->y);
    }
}

int main() {
    Point p = {10, 20};
    Circle c = {5.5, 90.0};

    print_shape(&p, 1); // Treat Point as a Circle (Type Confusion)
    return 0;
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    radius: f32,
    angle: f32,
}

fn print_shape_point(point: &Point) {
    println!("Point: x = {}, y = {}", point.x, point.y);
}

fn print_shape_circle(circle: &Circle) {
    println!("Circle: radius = {:.2}, angle = {:.2}", circle.radius, circle.angle);
}

fn main() {
    let p = Point { x: 10, y: 20 };
    let c = Circle { radius: 5.5, angle: 90.0 };

    // Rust won't allow this:
    // Compile-time error: mismatched types
    // print_shape_circle(&p);
    print_shape_circle(&p as Circle); // This would be a type confusion

    // OK
    print_shape_point(&p);
    print_shape_circle(&c);
}
```

```bash
Errors

Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0605]: non-primitive cast: `&Point` as `&Circle`
  --> src/main.rs:25:23
   |
25 |     print_shape_circle(&p as &Circle);
   |                       ^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or pointers
   |
   = note: `as` casts can only be used between primitive types (like `i32` and `f64`) or pointers (`*const T`, `*mut T`), not references (`&T`, `&mut T`) directly between different types. Use function calls or type-specific methods for conversions.

For more information about this error, try `rustc --explain E0605`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```

```rust
fn takes_u32(value: u32) {
    println!("Value: {}", value);
}

fn main() {
    // this is u8 value (Size is 1 byte)
    let data: u8 = 255;
    takes_u32(data); // Pass `u8` to a function expecting `u32`
}
```

```bash
Errors

Exited with status 101

Standard Error

Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:9:15
  |
9 |     takes_u32(data); // Pass `u8` to a function expecting `u32`
  |     --------- ^^^^ expected `u32`, found `u8`
  |     |
  |     arguments to this function are incorrect
  |
note: function defined here
 --> src/main.rs:2:4
  |
2 | fn takes_u32(value: u32) {
  |    ^^^^^^^^^ ----------
help: you can convert a `u8` to a `u32`
  |
9 |     takes_u32(data.into()); // Pass `u8` to a function expecting `u32`
  |                   +++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to 1 previous error
```