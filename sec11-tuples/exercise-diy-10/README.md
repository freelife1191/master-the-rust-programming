# Exercise-diy-10

현재 시스템의 작동 모드와 상태에 따라 명령을 실행하기 위해 긴 `if-else-if` 래더를 사용하는 Rust 함수 `execute_command_original`이 제공되었습니다

귀하의 과제는 이 코드를 리팩터링하는 것입니다. '튜플 패턴 일치'를 구현하면 코드를 읽기 쉬울 뿐만 아니라 더욱 우아하게 만들 수 있습니다

리팩터링된 코드는 아래 코드처럼 모드와 상태의 모든 조합을 올바르게 처리해야 합니다

```rust
fn execute_command_original(mode: &str, status: &str) {
    if mode == "admin" && status == "active" {
        println!("Admin privileges granted. Executing active command.");
    } else if (mode == "normal" && status == "pending") || (mode == "normal" && status == "active")
    {
        println!("Normal operation. Execute pending or active commands.");
    } else if mode == "maintenance" && status == "complete" {
        println!("System maintenance complete. Ready for normal operation.");
    } else {
        println!("No action needed or invalid mode/status.");
    }
}

//Write the refactored code using tuple and pattern matching using 'match' statement
fn execute_command_refactored(mode: &str, status: &str) {
    match (mode, status) {
        ("admin", "active") => {
            println!("Admin privileges granted. Executing active command.");
        }

        ("normal", "pending") | ("normal", "active") => {
            println!("Normal operation. Execute pending or active commands.");
        }

        ("maintenance", "complete") => {
            println!("System maintenance complete. Ready for normal operation.");
        }

        _ =>  println!("No action needed or invalid mode/status."),
    }
}

fn main() {
    execute_command_refactored("admin", "active");
    execute_command_refactored("normal", "pending");
    execute_command_refactored("maintenance", "complete");
    execute_command_refactored("admin", "pending");
}
```