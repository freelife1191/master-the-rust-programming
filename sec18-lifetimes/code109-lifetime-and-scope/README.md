
# Lifetime Vs Scope

```rust
fn main() {
    // Scope of 'reference'
    let reference;
    // Scope of 'value' (This is also the lifetime)
    {
        let value = 10;
        reference = &value;
    }

    // Error
    println!("Value at reference: {}", reference);
}
```