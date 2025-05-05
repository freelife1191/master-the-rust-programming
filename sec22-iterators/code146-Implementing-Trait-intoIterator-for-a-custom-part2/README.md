# Implementing Trait `IntoIterator` for a custom type (Part 2)

Rust의 `IntoIterator` 특성 자체는 반복자에서 다음 값을 얻는 방법을 정의하지 않습니다.  
대신, 주요 역할은 유형을 반복자로 변환하는 것입니다.  
시퀀스의 다음 값을 가져오는 실제 메커니즘은 `Iterator` 특성에 의해 정의됩니다.