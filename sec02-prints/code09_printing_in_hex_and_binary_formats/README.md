# 부동 소수점 값 인쇄

```rust
fn main() {
    // 사용자 정의 소수점 자리로 부동 소수점 값을 인쇄합니다
    let real_value = 3.14159;
    println!("With 2 decimal places value would be {:.2}", real_value);
    println!("With 6 decimal places value would be {:.6}", real_value);
    println!(
        "int part of the real value {} is {}",
        real_value,
        real_value as i32 // 'as' does casting
    );
}
```

`:.2` and `:.6` are format specifiers

Output:

```shell
With 2 decimal places value would be 3.14
With 6 decimal places value would be 3.141590
int part of the real value 3.14159 is 3
```


## 16진수로 10진수를 인쇄하세요

Rust 에서 10진수를 16진수로 변환하고 인쇄하려면 `:#X`, `:#x`, `:x`, `:X` 형식 지정자와 함께 `print` 또는 `format` 매크로를 사용할 수 있습니다


```rust
fn main() {
    // 16진수 스타일로 인쇄
    let decimal_num = 6789;
    let output1 = format!("decimal number {} in hex is {:#X}", decimal_num, decimal_num);
    let output2 = format!("decimal number {} in hex is {:#x}", decimal_num, decimal_num);
    let output3 = format!("decimal number {} in hex is {:x}", decimal_num, decimal_num);
    println!("{}\n{}\n{}", output1, output2, output3);
}
```

Output:

```shell
decimal number 6789 in hex is 0x1A85
decimal number 6789 in hex is 0x1a85
decimal number 6789 in hex is 1a85
```


## 숫자를 이진 형식으로 인쇄하기

`:b` 또는 `:#b` 형식 지정자를 사용하세요  
`#b` 형식 지정자는 `0b` 접두사가 있는 이진수를 인쇄하는 데 사용됩니다


```rust
fn main() {
    // print in binary
    println!("decimal number {num} in binary is {num:#b}", num = 6789);
    // println!("decimal number {} in binary is {:#b}", 6789, 6789);
}
```


```shell
decimal number 6789 in binary is 0b1101010000101
```