# Smart Pointers and Raw Pointers

## Raw pointers in Rust

```c
#include <stdio.h>

int main() 
{
    int x = 5;
    int* raw_ptr = &x;
    printf("raw_ptr points to: %d\n", *raw_ptr);
    *raw_ptr = 10;
    printf("raw_ptr points to: %d\n", *raw_ptr);
    return 0;
}
```

A mutable `pointer` in 'C'

```c
int main(void) {
    int x = 10; //let mut x = 10;
    int* raw_ptr = &x; // let mut raw_ptr = &mut x as *mut i32
    printf("%d\n", *raw_ptr);
    *raw_ptr = 5;
    printf("%d\n", *raw_ptr);
    return 0;
}
```

```bash
10
5
```

```rust
fn main() {
    let mut x = 10;
    let mut raw_ptr = &mut x as *mut i32; // 가변 참조를 가변 원시 포인터로 변환

    // 주의: 원시 포인터 역참조는 unsafe 블록 안에서만 가능합니다.
    // 아래 코드가 컴파일되려면 unsafe { ... } 로 감싸야 합니다.
    println!("{}", *raw_ptr); // unsafe 필요: raw_ptr 역참조
    *raw_ptr = 5;            // unsafe 필요: raw_ptr 역참조하여 쓰기
    println!("{}", *raw_ptr); // unsafe 필요: raw_ptr 역참조
}
```

```bash
Compiling playground v0.0.1 (/playground)
error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
 --> src/main.rs:5:20
  |
5 |     println!("{}", *raw_ptr);
  |                    ^^^^^^^^ dereference of raw pointer
  |
  = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or
```

## 왜 raw `pointer` 를 역참조하는 것이 unsafe 인가?

* Rust는 가리키는 데이터(참조 대상, referent)의 `lifetime` 이 유효하다고 보장할 수 없음 데이터가 이미 할당 해제(deallocated)되었거나 이동(moved)했을 수 있음
* raw `pointer` 는 유효하지 않은 메모리 위치를 가리킬 수 있으며, 역참조 시 `undefined behavior` 로 이어질 수 있음
* `references` (`&T` 또는 `&mut T`)와 달리, raw `pointers` (`*const T` 또는 `*mut T`)는 `borrow checker` 에 의해 추적되지 않음 Rust의 `ownership` 및 `borrowing rules` 가 적용되지 않아, 배타적(exclusive) 또는 공유(shared) 접근 규칙이 유지됨을 보장하는 것이 불가능함
* raw `pointer` 가 예상되는 `type` 과 일치하지 않는 데이터를 가리킬 수 있음
* `synchronization` 없이 raw `pointer` 를 역참조하면 `multithreaded` `context` 에서 `data race` 로 이어질 수 있음

```rust
// *mut T to create a mutable raw pointer in Rust
// *const T to create a readonly raw pointer in rust | (주석 잘림)
fn main() {
    let mut x = 10;
    // raw_ptr 자체가 변경되는 것은 아니므로 mut이 꼭 필요하진 않음
    let raw_ptr = &mut x as *mut i32;
    unsafe { // 원시 포인터 역참조는 unsafe 블록 안에서 수행
        println!("{}", *raw_ptr);
        *raw_ptr = 5;
        println!("{}", *raw_ptr);
    }
}
```

## Types of raw pointers

1.  **Mutable Raw Pointer (`*mut T`):**
    * 이 raw `pointer` 는 가리키는 데이터의 변경(mutation)을 허용함 `borrowing` 및 안전성(safety) 보장 없이 가변 참조(`&mut T`)와 직접적으로 대응됨
2.  **Immutable Raw Pointer (`*const T`):**
    * 이것은 데이터가 이 `pointer` 를 통해 수정되어서는 안 된다는 의도를 가진 데이터를 가리키는 데 사용되는 raw `pointer` 임 C 와 C++ 같은 언어의 "const `pointer`"와 개념적으로 유사함

**Note:** `*const T` 가 가리키는 데이터는 본질적으로 상수(constant)이거나 불변(immutable)인 것은 아님 오히려, `*const T` 는 해당 `pointer` 를 통해서는 데이터가 수정되어서는 안 된다는 의도를 표현하는 방법임

```c
#include<stdio.h>

//lets create a pointer which points to a data but the intention is
//that the data should not be modified through the pointer.
// this type of pointer is called as const pointer in C
// 데이터는 가리키지만 포인터를 통해 데이터를 수정하지 못하도록 하는 포인터를 만들어 봅시다.
// C에서는 이런 종류의 포인터를 const 포인터라고 부릅니다. (정확히는 pointer-to-const)

int main(void) {
    int x = 10;
    const int *ptr = &x; // 이것이 C에서 pointer-to-const (상수에 대한 포인터)를 만드는 방법입니다.
                         // ptr 자체는 변경 가능하지만, ptr을 통해 x의 값을 변경할 수는 없습니다.
    // *ptr = 30;        // 컴파일 오류 발생! ptr이 가리키는 대상(*ptr)은 읽기 전용(read-only)입니다.
    printf("%d\n", x); // *ptr=30 라인이 없거나 주석 처리되면 10이 출력됩니다.
    return 0;          // main 함수의 표준 반환값
}
```

```bash
main.c: In function 'main':
main.c:10:10: error: assignment of read-only location '*ptr'
   10 |     *ptr = 30;
      |          ^
```

```rust
//lets create a pointer which points to a data but the intention is
//that the data should not be modified through the pointer.
// this type of pointer is called as const pointer in C
// 데이터는 가리키지만 포인터를 통해 데이터를 수정하지 못하도록 하는 포인터를 만들어 봅시다.
// C에서는 이런 종류의 포인터를 const 포인터라고 부릅니다. (정확히는 pointer-to-const)
fn main() {
    let mut x = 10;
    let ptr = &x as *const i32; //const int *ptr = &x;
    
    *ptr = 30;
    printf("%d\n", x);
}
```

```c
#include<stdio.h>

//lets create a pointer which points to a data but the intention is
//that the data should not be modified through the pointer.
// this type of pointer is called as const pointer in C
// 데이터는 가리키지만 포인터를 통해 데이터를 수정하지 못하도록 하는 포인터를 만들어 봅시다.
// C에서는 이런 종류의 포인터를 const 포인터라고 부릅니다. (정확히는 pointer-to-const)

int main(void) {
    int x = 10;
    int y = 50;
    const int *const ptr = &x; // 이것이 C에서 pointer-to-const (상수에 대한 포인터)를 만드는 방법입니다.
    ptr = &y;
    // *ptr = 30;        // 컴파일 오류 발생! ptr이 가리키는 대상(*ptr)은 읽기 전용(read-only)입니다.
    printf("%d\n", x); // *ptr=30 라인이 없거나 주석 처리되면 10이 출력됩니다.
    return 0;          // main 함수의 표준 반환값
}
```

```bash
main.c: In function 'main':
main.c:11:9: error: assignment of read-only variable 'ptr'
   11 | ptr = &y;
      | ^
```

```rust
//lets create a pointer which points to a data but the intention is
//that the data should not be modified through the pointer.
// this type of pointer is called as const pointer in C
// 데이터는 가리키지만 포인터를 통해 데이터를 수정하지 못하도록 하는 포인터를 만들어 봅시다.
// C에서는 이런 종류의 포인터를 const 포인터라고 부릅니다. (정확히는 pointer-to-const)

fn main() {
    let mut x = 10; // const int x = 10;
    let ptr = &mut x as *const i32; //const int *const ptr = &x;
    unsafe {
        *ptr = 30;
        println!("{}", x);
    }
}
```

## Rust의 스마트 포인터와 비교했을 때 Raw 포인터의 부족한 점

1.  **안전 보장 없음:** Raw 포인터의 역참조는 본질적으로 안전하지 않습니다.
2.  **자동 메모리 관리 없음:** Raw 포인터는 메모리 할당 또는 해제를 관리하지 않습니다.
3.  **빌림(borrowing) 및 소유권(ownership) 강제 없음:** Raw 포인터는 Rust의 소유권 및 빌림 규칙을 우회합니다.
4.  **모호한 가변성:** 가변성 및 불변성 구분은 관례에 의해서만 이루어집니다.
5.  **라이프타임(Lifetimes) 없음:** Raw 포인터는 라이프타임을 강제하지 않아 매달린 포인터(dangling pointers) 위험이 있습니다.
6.  **데이터 경쟁(Data races)에 취약:** 여러 개의 가변 Raw 포인터가 검사 없이 동일한 위치를 가리킬 수 있습니다.
7.  **참조 카운팅(Reference counting) 없음:** `Rc<T>` 또는 `Arc<T>`와 달리 Raw 포인터는 참조를 추적하지 않습니다.
8.  **실행 시간(Runtime) 빌림 검사 없음:** `RefCell<T>`와 달리 Raw 포인터는 실행 시간에 빌림 규칙을 검사하지 않습니다.
9.  **미정의 동작(Undefined behavior)의 직접적인 위험:** 잘못 사용하면 컴파일러 경고 없이 미정의 동작으로 이어질 수 있습니다.
10. **스레드 안전(Thread-safety) 보장 없음:** Raw 포인터 자체는 스레드 안전에 대한 어떠한 보장도 제공하지 않습니다.


## Raw 포인터 사용 사례

- **C 및 다른 저수준 시스템 코드와의 상호 운용성:** 시스템 프로그래밍에서 기존 C 라이브러리 또는 운영체제 API와 인터페이스하기 위해

