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