# Exercise

주어진 파일의 이름을 바꾸는 함수를 작성하세요

- 이름 변경에 성공하면 `Ok(())`를 반환합니다
- 해당 파일이 존재하지 않으면 `Err(std::io::Error)`를 반환합니다


## std::io::Result

`std::io` 모듈에서 유형 정의를 찾을 수 있습니다:

`pub type Result<T> = Result<T, Error>;`

`std::io::Result`는 Rust의 표준 Result 유형에 대한 유형 별칭이며, 오류 유형은 `std::io::Error`로 고정되어 있습니다  
이 유형 별칭의 목적은 I/O 오류를 반환할 수 있는 결과 값으로 작업하기를 더 쉽게 만드는 것입니다

`std::io::Error` 유형의 오류를 반환하는 경우 표준 `Result<T, E>` 유형 또는 `std::io::Result<T>` 유형을 사용할 수 있습니다  
두 유형 모두 본질적으로 동일하며 유일한 차이점은 오류 유형입니다  
표준 `Result<T, E>` 유형은 모든 유형 `E`를 오류로 보유할 수 있는 반면 `std::io::Result<T>` 유형은 유형입니다  
`Result<T, std::io::Error>`에 대한 별칭입니다  
이는 `std::io::Error` 개체만 오류로 보유할 수 있음을 의미합니다

따라서 함수가 `Result<T, E>`(여기서 `E`는 `std::io::Error`)를 반환하는 경우 대신 `std::io::Result<T>` 유형 별칭을 사용하여 코드를 더 간결하게 만들 수 있습니다  
읽기가 더 쉽습니다


## Summary

- `std::io::Error`는 Rust 표준 라이브러리의 io 모듈에서 제공하는 오류 유형입니다  
  `std::io::Error` 유형은 파일이나 네트워크 소켓 읽기 또는 쓰기와 같은 I/O 작업 중에 발생하는 오류를 나타내는 데 사용됩니다
- `std::io::Error` 유형을 자신의 Result 유형에 직접 사용하거나 io 모듈에서 제공하는 `std::io::Result` 유형 별칭을 사용할 수 있습니다. `Result<T, std::io::Error>`
- rename은 표준 라이브러리의 fs 모듈에서 제공하는 기능으로, 이를 사용하려면 use 문을 사용하여 fs 모듈을 import 해야 합니다
- 코드 시작 부분에 `use std::fs;`를 포함하면 fs 모듈에 정의된 모든 항목을 현재 범위로 가져옵니다