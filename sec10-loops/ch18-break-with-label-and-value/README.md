## break with label and return value

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