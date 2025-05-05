# `Box<T>` `Borrowing`

`into_raw()` 함수는 `Box<T>`를 소비하여 `heap`에 할당된 데이터의 `ownership`을 `raw pointer`에게 넘겨준다 사실상 `Box`를 잊는 것이다

`from_raw()` 함수는 `raw pointer`로부터 `Box<T>`를 재구성하는 데 사용된다 `ownership`을 되찾고 `Rust`가 `memory`를 다시 관리하도록 허용한다

> 만약 우리가 `raw pointer`를 가지고 일시적으로 작업하기를 원한다면 `Box<T>`를 `Borrowing`하는 것은 어떤가?

```rust
fn main() {
    let mut boxed_int = Box::new(10);
    //borrow Box as immutable raw pointer
    let immutable_raw_ptr = &(*boxed_int) as *const i32;

    //borrowing Box as mutable raw pointer
    let mutable_raw_ptr = &mut (*boxed_int) as *mut i32;
    
    unsafe {
        *mutable_raw_ptr = 30;
        println!("{}", *mutable_raw_ptr);
    }

    unsafe {
        println!("{}", *immutable_raw_ptr);
    }
}
```

```bash
30
30
```