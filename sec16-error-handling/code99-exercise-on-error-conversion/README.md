# Exercise

사용자로부터 숫자를 문자열로 받아들이고, 문자열을 구문 분석하여 숫자로 변환한 후 `Ok(number)`를 반환하는 함수를 작성하라  
구문 분석에 실패하면 `ErrorKind`를 `InvalidData`로 설정하여 `Err(std::io::Error)`를 반환한다

- https://doc.rust-lang.org/stable/std/io/index.html#functions
  - https://doc.rust-lang.org/stable/std/io/fn.stdin.html
  - https://doc.rust-lang.org/stable/std/io/struct.Stdin.html