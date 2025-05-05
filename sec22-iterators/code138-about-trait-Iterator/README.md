# Trait ‘Iterator’ vs Trait ‘IntoIterator’

Refer: https://doc.rust-lang.org/std/iter/trait.Iterator.html

**Trait ‘Iterator’**: 반복자를 정의하는 데 사용됩니다.  
반복자는 단계별 방식으로 일련의 값을 처리할 수 있는 유형입니다.  
이 특성의 기본 메소드는 시퀀스의 **next value**를 포함하는 **Option**을 반환하거나 처리할 값이 더 이상 없으면 **None**을 반환하는 next 메서드입니다.