## break with return value

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