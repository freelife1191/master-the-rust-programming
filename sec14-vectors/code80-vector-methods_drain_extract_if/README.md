## drain()

`Vec`의 `drain` 메소드는 범위인 단일 매개변수를 사용합니다  
이 범위는 배수될 벡터 섹션의 시작과 끝을 지정합니다.

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let _ = v.drain(1..3); // This will drain values 2 and 3
    println!("{:?}", v);
}
```

### Use Case Scenarios

1. 전체 벡터 할당을 해제하지 않고 벡터에서 요소 범위 제거
2. 한 벡터에서 다른 벡터로 요소의 하위 집합을 이동합니다
3. 벡터에서 요소를 한 번에 하나씩 제거
4. 5보다 큰 모든 요소 등 특정 요소 제거

install nightly

```shell
rustup toolchain install nightly
```

run

```shell
rustup run nightly rustc --version
```

default nightly

```shell
rustup default nightly
```

rustup update

```shell
rustup update
```