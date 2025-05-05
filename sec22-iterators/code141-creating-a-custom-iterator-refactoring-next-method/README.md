# Creating a custom iterator: Refactoring 'next' method

```rust
fn main() {
    let mut numbers = ["One".to_string(), "Two".to_string(), "Three".to_string()];

    /* `iter()` 메소드는 컬렉션의 요소(예: 배열 또는 벡터)에 대한 불변 참조를 생성하는 반복자를 생성합니다.
       `iter()`를 사용하면 요소를 읽을 수 있지만 수정할 수는 없습니다.
       반복자가 생성한 요소의 유형은 '&T'입니다.
       https://doc.rust-lang.org/stable/std/slice/struct.Iter.html
       https://doc.rust-lang.org/stable/std/primitive.slice.html#method.iter */
    let iterate_by_immutable_borrow = numbers[0..=1].iter();

    /* `iter_mut()` 메소드는 컬렉션의 요소(예: 배열 또는 벡터)에 대한 변경 가능한 참조를 생성하는 반복자를 생성합니다.
       `iter_mut()`를 사용하면 요소를 읽을 수 있고 수정할 수도 있습니다.
       반복자가 산출한 요소의 유형은 '&mut T'입니다.
       https://doc.rust-lang.org/stable/std/slice/struct.IterMut.html 
       https://doc.rust-lang.org/stable/std/primitive.slice.html#method.iter_mut */
    let iterate_by_mutable_borrow = numbers.iter_mut();

    /* `into_iter()` 메소드는 컬렉션을 소비하고 해당 요소를 값으로 생성하는 반복자를 생성합니다.
       `into_iter()`를 호출한 후에는 소유권이 반복자에게 이전되었기 때문에 원본 컬렉션을 사용할 수 없습니다.
       이 반복자가 생성한 요소의 유형은 'T'입니다. 
       https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
       https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html#tymethod.into_iter */
    let iterate_by_value = numbers.into_iter();

    // i의 유형은 `&mut String`입니다.
    for i in iterate_by_mutable_borrow {
        println!("{}", i);
    }

    println!("{:?}", numbers);
}
```

### find method

https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.find

```rust
fn main() {
    let a = [1, 2, -1];
    // item is &i32
    // let ret = a.iter().find(|x| x < 0);
    // item is &&i32
    // let ret = a.iter().find(|x| **x < 0); // Some(-1)
    // item is &&i32
    // let ret = a.iter().find(|&&x| x < 0); // Some(-1)
    println!("{:?}", ret);
}
```