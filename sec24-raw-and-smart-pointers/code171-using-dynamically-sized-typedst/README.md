# 동적 크기 유형 사용(DST)

`Box<T>` to handle Dynamically Sized Types(DSTs)

## 동적 크기 타입

Rust의 Dynamically Sized Types (DSTs)는 컴파일 시점에 크기를 알 수 없으며 런타임에서만 결정되는 타입이다

| 카테고리           | 예시                                  | 설명                                                                       |
| :----------------- | :------------------------------------ | :------------------------------------------------------------------------- |
| 일반 타입          | `i32`, `bool`, `char`, `struct`, `enum`, `[i32; 5]` | 이 타입들은 컴파일 시점에 결정되는 정적으로 알려진 크기를 가진다                 |
| 동적 크기 타입 (DSTs) | `[T]`, `dyn Trait`                   | 이 타입들은 컴파일 시점에 알려진 크기를 가지지 않으며 런타임에서만 완전히 이해될 수 있다 |


```rust
// Rust에서 `slices`는 `DSTs`이다
fn main() {
    let array1: [i32; 4] = [1, 2, 3, 4];
    let slice: [i32] = array1[0..=2];
}
```

```bash
Exited with status 101
Standard Error
Compiling playground v0.0.1 (/playground)
error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
--> src/main.rs:7:9
  |
7 | let slice: [i32] = array1[0..=2];
  |           ^^^^^ doesn't have a size known at compile-time
  |
= help: the trait `Sized` is not implemented for `[i32]`
```

```bash
`DSTs`는 컴파일 시점에 크기를 알 수 없기 때문에 직접 인스턴스화하거나 변수에 저장될 수 없다
```

```rust
// Rust에서 `slices`는 `DSTs`이다
fn main() {
    let array1: [i32; 4] = [1, 2, 3, 4];
    let slice: &[i32] = &array1[0..=2];
    println!("{:?}", slice);
}
```

```bash
[1, 2, 3]
```

### Dynamically Sized Types : Slices

`Dynamically Sized Types` (`DSTs`)는 `slices`와 같이 프로그램에서 활용되는 두 가지 주요 방법이 있다

1.  `&[T]` 또는 `&mut [T]`와 같은 참조를 통해서
2.  `Box<T>`와 같은 `smart pointer`를 통해서

### DST Boxing : Box<DST>

`Box`, `Rc`, `Arc`와 같은 일부 `smart pointer`는 `rust`에서 `DSTs`에게 `compile time`에 `statically known size`를 제공함으로써 `DSTs`를 관리하는 데 사용될 수 있다