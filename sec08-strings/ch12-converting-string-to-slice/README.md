# Convert String to slice and vice versa

```rust
fn main() {
    let s = String::from("hello");
    let slice2 = s.as_str();
    let slice = &s;
    
    println!("s = {}", s);
    println!("slice1 = {}", slice1);
    println!("slice2 = {}", slice2);
}
```

```rust
fn main() {
    let slice = "hello";
    let string1 = slice.to_string();
    let string2 = String::from(slice);

    println!("slice = {}", slice);
    println!("string1 = {}", string1);
    println!("string2 = {}", string2);
}
```