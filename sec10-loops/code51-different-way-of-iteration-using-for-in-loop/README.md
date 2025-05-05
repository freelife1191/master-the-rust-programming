### for .... in ... loop

- 반복자에 대한 `for` 루프 반복은 반복자가 `None`을 생성하면 중지됩니다. `None` 값은 반복기의 맥락에서 시퀀스의 끝을 나타냅니다.
- Rust 의 반복자는 `next()` 메서드를 정의하는 Iterator trait 을 기반으로 구축되었습니다  
  이 메소드는 `Some(value)` 또는 `None`일 수 있는 `Option` 유형을 반환합니다  
  반복자에 더 이상 산출할 값이 없으면 `next()`는 `Some(value)`를 반환하고 반복자에 더 이상 산출할 값이 없으면 `None`을 반환합니다

https://doc.rust-lang.org/stable/std/option/enum.Option.html