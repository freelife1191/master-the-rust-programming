# Rust가 메모리 안전(memory safe)하다고 말하는 것은 무엇을 의미하는가? Part-2

**메모리 관련 오류를 유발하는 방식으로 코드를 작성할 수 없음**

* ❌ 널 포인터 역참조 (Null pointer dereference)
* ❌ 해제 후 사용 (Use after free)
* ❌ 댕글링 포인터 (Dangling pointers)
* ❌ 이중 해제 (Double free)
* ❌ 버퍼 오버플로우 (Buffer overflows)
* ❌ 메모리 누수 (Memory Leaks) (대부분 방지됨; 어떤 언어도 모든 시나리오에서 메모리 누수를 완벽하게 막을 수는 없음)
* ❌ 데이터 경쟁 (Data races)
* ❌ 초기화되지 않은 메모리 접근 (Uninitialized memory access)
* ❌ 타입 혼동 (Type confusion)

> 언어의 핵심 원칙 덕분에 안전한 Rust(`safe Rust`)에서는 메모리 불안전 코드를 작성하는 것이 불가능함

## 버퍼 오버플로우 (Buffer overflow)

* **`C/C++`**
    * 정의되지 않은 동작 (표준에 따름)

* **`Rust`:**
    * 정의된 동작
    * 빌드 모드(디버그 또는 릴리스)에 관계없이 프로그램 패닉 발생

```rust
fn main() {
    let buffer = [1, 2, 3, 4, 5];

    for i in 0..10 {
        println!("{}", buffer[i]);
    }
}
```

```bash
1
2
3
4
5
```

## 패닉(Panic)은 크래시(Crash)와 같지 않음

**패닉 (Panic)**

* 프로그램을 정상적으로 종료함
* 스택을 풀어서 리소스를 정리함 (메모리 해제 등)
* 정의되지 않은 동작이 없도록 보장함

**크래시 (Crash)**

* 갑작스럽고 제어되지 않는 종료임
* 정의되지 않은 동작
* 리소스를 일관되지 않은 상태로 남겨둘 수 있음

```rust
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};

static FAIL_SAFE_MODE: AtomicBool = AtomicBool::new(false);

fn main() {
    // give custom panic hook
    panic::set_hook(Box::new(|info| {
        FAIL_SAFE_MODE.store(true, Ordering::SeqCst);
        println!("Panic occurred: {}", info);
        println!("Entering fail-safe mode...");
    }));

    // Wrap the potentially panicking code in catch_unwind
    let result = panic::catch_unwind(|| {
        let buffer = [1, 2, 3, 4, 5];
        for i in 0..10 {
            // panics for i >= 5
            println!("Accessing index {}: {}", i, buffer[i]);
        }
    });

    // Check if the application is in fail-safe mode
    if FAIL_SAFE_MODE.load(Ordering::SeqCst) {
        println!("System is now in fail-safe mode.");
    }

    // Printing result of the catch
    match result {
        Ok(_) => println!("No panic occurred."),
        Err(_) => println!("Panic caught! Execution continues."),
    }
}
```

```bash
Accessing index 0: 1
Accessing index 1: 2
Accessing index 2: 3
Accessing index 3: 4
Accessing index 4: 5
Panic occurred: panicked at src/main.rs:18:51:
index out of bounds: the len is 5 but the index is 5
Entering fail-safe mode...
System is now in fail-safe mode.
Panic caught! Execution continues.
```