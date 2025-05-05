## Auto conversion of errors

```rust
use std::ffi::{CString, NulError};
use std::io;

/*
단위 유형으로 결과 유형을 반환하는 함수 정의()
성공을 나타내는 NulError 또는 실패를 나타내는 NulError

https://doc.rust-lang.org/stable/std/io/struct.Error.html#impl-From%3CNulError%3E-for-Error
Return type: Result<(), NulError>
 */
fn some_function() -> Result<(), NulError> {
    /*
    CString::new()를 사용하여 C 스타일 문자열 만들기
    null 종결자 "\0"이 있는 문자열 리터럴
     */
    let c_string = CString::new("foo\0bar")?;
    Ok(())
}

/*
`main()` 함수에서 `std::io::Error`에 대한 `From` 특성 구현은 다음에 지정된 대로 `NullError`를 `std::io::Error` 유형으로 변환하는 데 사용됩니다. `main()`의 반환 유형.

Return type: Result<(), std::io::Error>
 */
fn main() -> io::Result<()> {
    some_function()?;
    Ok(())
}

```


## 그렇다면 오류 유형의 자동 변환은 언제 가능합니까?

원래 오류 유형에서 원하는 오류 유형으로 변환하는 데 사용할 수 있는 `From` 특성의 구현이 있을 때 오류 유형의 자동 변환이 가능합니다


```rust
use std::ffi::{CString, NulError};
use std::io;

fn some_function() -> io::Result<()> {
    let c_string = CString::new("foo\0bar")?;
    Ok(())
}

/*
`NulError` 유형에서 `std::io::Error`를 `NulError`로 변환하는 명시적인 From 특성 구현이 없기 때문에 Rust가 자동으로 변환을 수행할 수 없기 때문에 컴파일 오류가 발생합니다
 */
fn main() -> Result<(), NulError> {
    some_function()?;
    Ok(())
}
```