# Implementing Trait `IntoIterator` for a custom type (Part 3)

```rust
// Custom iterator for iteration by-value
struct CarPriceRangeIteratorByValue {
    // https://doc.rust-lang.org/stable/std/vec/struct.IntoIter.html
    remaining_cars: std::vec::IntoIter<Car>,
    price_range: (u32, u32),
}

// Custom iterator for iteration by-immutable-reference
struct CarPriceRangeIteratorByRef<'a> {
    // https://doc.rust-lang.org/stable/std/slice/struct.Iter.html
    remaining_cars: std::slice::Iter<'a, Car>,
    price_range: (u32, u32),
}

// Custom iterator for iteration by-mutable-reference
struct CarPriceRangeIteratorByMutRef<'a> {
    // https://doc.rust-lang.org/stable/std/slice/struct.IterMut.html
    remaining_cars: std::slice::IterMut<'a, Car>,
    price_range: (u32, u32),
}
```

