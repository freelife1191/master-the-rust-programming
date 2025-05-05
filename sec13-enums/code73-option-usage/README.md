# Exercise

문자열 배열에서 가장 긴 문자열 항목을 반환하는 함수를 작성하세요
반환 값은 `Option` 유형이어야 합니다
배열이 비어 있으면 `None`, 그렇지 않으면 `Some(longest_string)`입니다


- Array of i32: `[34, 56, 78, 89]`
- Array of &str: `["abc", "efg", "xyz", "123"]`

`&[i32; 4]`(고정 크기 배열에 대한 참조)를 `&[i32]`(슬라이스에 대한 참조)를 기대하는 함수로 변환합니다  
Rust는 자동으로 배열 참조를 슬라이스 참조로 강제 변환합니다.

- https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none
- https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
- https://doc.rust-lang.org/std/result