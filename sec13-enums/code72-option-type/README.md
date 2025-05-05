## The 'Option' enum type

`Option` 열거형은 값이 있거나 없을 가능성을 캡슐화합니다  
이는 Rust 의 `Some(value)` 또는 `None`일 수 있는 선택적 값의 개념을 나타냅니다

`Option` 유형은 Rust 표준 라이브러리 `std::option::Option`에 정의되어 있습니다

https://doc.rust-lang.org/std/option/enum.Option.html


Rust 는 `Option`을 사용하여 개발자가 `null` 허용 여부를 명시적으로 처리하도록 권장하고 코드에 값이 없을 가능성을 설명하도록 강제합니다  
이 접근 방식은 널 참조 오류를 방지하는 데 도움이 되며 더욱 안전하고 강력한 소프트웨어를 장려합니다


## The Rust Prelude

Rust 에서는 `Option` 유형이 서문에 포함되어 있으므로 `use` 키워드를 사용하여 이를 명시적으로 가져올 필요가 없습니다    
prelude 는 모든 Rust 모듈에 자동으로 가져오는 항목 세트입니다

https://doc.rust-lang.org/std/prelude/index.html