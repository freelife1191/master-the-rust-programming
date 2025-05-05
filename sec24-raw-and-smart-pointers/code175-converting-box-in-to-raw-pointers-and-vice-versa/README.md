# `Box<T>`를 `raw pointer`로 변환하기 그리고 그 반대

**사용 사례: `Foreign Function Interface` (`FFI`) with `C`**

`Rust`에서 `C functions`를 `calling`할 때, `data`를 `pass`하기 위해 일반적으로 `raw pointers`(`*mut T` 또는 `*const T`)를 사용한다

그리고 `C functions`로부터 `data`를 `receiving`할 때, `raw pointers`로 `receive`할 수 있으며, `Rust`의 `ownership model` 내에서 안전하게 `manage`하기 위해 이 `raw pointers`를 `Rust's types`으로 다시 `convert`하려 할 수 있다

## Box::into_raw()` 와 `Box::from_raw()`

`Box::into_raw()`는 `Box<T>`를 소비하여 `heap`에 할당된 `T value`에 대한 `raw, mutable pointer`를 반환하는 메서드이다

```rust
let boxed_int = Box::new(123);
let raw_ptr_to_int: *mut i32 = Box::into_raw(boxed_int);
```

`Rust`의 `safe memory management`는 여기에 적용되지 않는다. `memory`는 수동으로 관리되어야 한다

## Rust에서 raw pointer deallocating하기

- `Box::into_raw()` 로부터 얻은 `raw pointer`로 시작할 때, `rust`에서 `memory`를 `deallocate`하는 가장 `safest`하고 `idiomatic`한 방법은 `Box::from_raw()` 를 사용하여 `Box` 로 다시 `convert`하고 `automatic deallocation`을 위해 `scope`를 벗어나게 하는 것이다
- `Box` 를 사용하지 않고 수동으로 `memory`를 `deallocate`하려고 시도하는 것은 `memory leaks`, `double frees`, 또는 `undefined behavior`를 포함한 불필요한 `errors` 위험을 초래한다

```rust
fn main() {
    let boxed_int = Box::new(100);
    // `constant raw pointer`
    let raw_ptr: *const i32 = Box::into_raw(boxed_int) as *const i32;

    unsafe {
        let _ = Box::from_raw(raw_ptr as *mut i32);
    }

    unsafe {
        println!("{}", *raw_ptr);
    }
}
```

```bash
0
```

```rust
// `Vec<i32>`를 `Box<[i32]>`로 변환하기
fn main() {
    let vec_of_i32: Vec<i32> = vec![10; 100];
    let boxed_i32: Box<[i32]> = vec_of_i32.into_boxed_slice();

    println!("{:?}", boxed_i32);
}
```

여기서, `dynamically resizable vector`에서 `fixed-size slice`로 이동했다, 그 크기를 고정하고 `vector`의 `capacity`에 대한 추가 수정을 방지한다

```bash
[10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, ....
```

## `Vec<T>` 대 `Box<[T]>`

* 늘어나거나 줄어들 수 있는 동적 크기 컬렉션이 필요할 때 `Vec<T>`를 사용한다
* `Vec<T>`는 `compile time`에 요소의 수가 알려져 있지 않거나 `execution` 중에 변경될 수 있는 상황에 이상적이다
* 크기가 알려져 있고 할당 후 변경되지 않는 고정 크기 컬렉션에 대해 `Box<[T]>`를 사용한다
* `boxed slice` (`[T]`)는 `fixed number of elements`를 가진 `heap-allocated array`를 나타내는 데 유용하다