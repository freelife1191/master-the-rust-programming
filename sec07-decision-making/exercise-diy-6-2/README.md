# Exercise-diy-6.2

**32비트 RGBA8888 색상 형식을 16비트 RGB565 색상 형식으로 변환**  
사용자로부터 16진수 32비트 RGBA8888 색상 형식을 받아들이는 프로그램을 작성하세요  
이를 16비트 RGB565 색상 형식으로 변환합니다  

힌트  
============================

1. 32비트 RGBA 입력에서 빨간색, 녹색, 파란색 구성요소를 추출합니다
2. RGB565 형식에 맞게 이러한 색상 구성 요소를 축소합니다  
   빨간색과 파란색에 5비트를 할당하고 녹색에 6비트를 할당합니다  
   예를 들어 0xABCDEFEE가 RGBA8888 형식인 경우 바이너리에서는 다음과 같습니다  
   `10101011(R) 11001101(G) 11101111(B) 11101110(A)`  
   이것을 RGB565로 변환하려면,  
 
   i) 방치 A  
   ii) R에서는 최상위 5비트만 고려  
   iii) G에서는 최상위 6비트만 고려  
   iv) B에서는 최상위 5비트만 고려  

3. 사용자 입력에서 16진수 접두사 제거:  
   문자열에서 'trim_start_matches' 메서드를 사용하여 "0x" 또는 "0X" 접두사를 제거합니다.
4. 문자열을 정수 U32 값으로 변환하려면 다음을 탐색하십시오.  
   `u32::from_str_radix() with radix = 16`


예상 출력  

```
=============================
RGBA8888 데이터를 16진수 형식(0xABCDEFEE)으로 입력하세요.
0xABCDEFEE RGB565에 해당하는 것은 0xAE7D입니다.
```


```rust
use std::io;
//We are using methods(like flush()) from the 'Write' trait of the std::io module
use std::io::Write;

fn read_user_input() -> String {
    //TODO
}

fn parse_hex_to_u32(hex_str: &str) -> u32 {
    //TODO
}

fn rgba8888_to_rgb565(rgba: u32) -> u16 {
    // TODO: Extract red, green, and blue components


    // TODO: Convert to RGB565


}

fn main() {
    print!("Enter RGBA8888 data in hex format:");
    //flushes the buffer of the standard output
    io::stdout().flush().expect("Failed to flush stdout");

    let input = read_user_input();
    let rgba888 = parse_hex_to_u32(&input);
    let rgb565 = rgba8888_to_rgb565(rgba888);
    println!("{:#x} in rgb565 format is {:#x}", rgba888, rgb565);
}
```