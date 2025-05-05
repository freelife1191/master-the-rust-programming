## Numeric Errors : std::num::ParseIntError

정수를 구문 분석할 때 반환될 수 있는 오류입니다


## Exercise

`string(&str)`을 입력으로 받아 이를 **32-bit signed integer (i32)** 로 구문 분석하는 `parse_integer_from_string` 이라는 함수를 구현하라 함수는 결과 유형을 반환해야 한다  
여기서 성공적인 구문 분석은 구문 분석된 정수 값 **num**을 사용하여 `Ok(num)`으로 표시되고, 실패한 구문 분석은 `Err(e)`로 표시된다 **e**는 해당 error message(**String**)이다

`parse_integer_from_string` 함수 내에서 제공된 문자열은 `i32::from_str` 메서드를 사용하여 구문 분석된다   
구문 분석에 성공하면 함수는 구문 분석된 정수 값을 반환해야 한다
그러나 파싱 중 오류가 발생하면 `from_str()` 메소드는 `std::num::parsingIntError` 유형의 오류 값을 반환한다  
다양한 오류 사례를 처리하려면 `std::num::parseIntError`의 `kind()` 메서드를 사용하라


```rust
use std::str::FromStr;

fn parse_integer_from_string(input: &str) -> Result<i32, String> {
    //TODO
}

fn main() {
    let result = parse_integer_from_string("9");
    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error parsing number: {}", e),
    }
}
```