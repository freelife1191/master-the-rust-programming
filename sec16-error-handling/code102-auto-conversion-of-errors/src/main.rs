use std::ffi::{CString, NulError};
use std::io;

/*
단위 유형으로 결과 유형을 반환하는 함수 정의()
성공을 나타내는 NulError 또는 실패를 나타내는 NulError

https://doc.rust-lang.org/stable/std/io/struct.Error.html#impl-From%3CNulError%3E-for-Error
Return type: Result<(), NulError>
 */
fn some_function() -> Result<(), NulError> {
// fn some_function() -> io::Result<()> {
    /*
    CString::new()를 사용하여 C 스타일 문자열 만들기
    null 종결자 "\0"이 있는 문자열 리터럴
     */
    let c_string = CString::new("foo\0bar")?;
    Ok(())
}

/*
오류 자동 변환
그렇다면 오류 유형의 자동 변환은 언제 가능한가?

Return type: Result<(), std::io::Error>
 */
fn main() -> io::Result<()> {
// fn main() -> Result<(), NulError> {
    some_function()?;
    Ok(())
}
