# Trait 'Iterator' vs. trait 'IntoIterator'

## 현재 구현의 한계

- **Immutable Iteration**: BookingOnDate 반복자는 예약(`&Booking`)에 대한 변경 불가능한 참조를 생성합니다.  
  즉, 예약을 반복하는 동안 예약을 수정할 수 없습니다.
- **No Mutable Iteration**: 예약을 변경해야 하는 경우(예: guest_name 또는 room_number 변경) 이 반복자는 변경 가능한 반복을 지원하지 않으므로 충분하지 않습니다.


## 대체 접근 방식(Alternative Approach): 별도의 반복자 구조 및 'Intolterator'

- **관심사의 분리(Separation of Concerns)**: 반복자에 대해 별도의 구조를 갖고 `Booking` 컬렉션에 대해 `Intolterator`를 구현함으로써 다양한 요구에 맞는 다양한 반복자를 만들 수 있습니다(예: 불변 반복자, 변경 가능 반복자 등).
- **유연성과 재사용성(Flexibility and Reusability)**: 이 접근 방식은 컬렉션을 반복하는 방법에 더 많은 유연성을 제공하고 동일한 데이터 구조(`Booking` 컬렉션)를 다양한 목적을 위해 다른 반복자와 함께 재사용할 수 있도록 합니다.


## Trait ‘Iterator’ vs Trait ‘IntoIterator’

- The `Iterator` trait provides the `next` method, which is used to retrieve the next item from an iterator.
- The `IntoIterator` trait provides the `into_iter` method, which converts a type into an iterator (`IntoIterator` is used for types that can be transformed into iterators).

> Rust 표준 라이브러리는 `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet`, `arrays` 등과 같은 많은 일반적인 유형에 대해 `IntoIterator`를 구현하여 `into iterators` 변환할 수 있도록 했습니다.


## The IntoIterator Trait

- `value`(i.e., `self`)으로 유형을 구현하면 해당 유형을 사용하여 반복자를 생성할 수 있습니다.
- `immutable reference`(i.e., `&self`)로 유형을 구현하면 원본 객체를 소비하지 않고도 불변 참조를 생성하는 반복자를 생성할 수 있습니다.
- `mutable reference`(i.e., `&mut self`)로 유형에 대해 구현되면 변경 가능한 참조를 생성하고 원본 객체를 변경하는 반복자를 생성할 수 있습니다.

## Note: Iterator Vs IntoIterator

- `Iterator`를 구현할 때 "이 유형은 어떻게 시퀀스에서 값을 생성합니까?"라는 질문에 답하게 됩니다.
  - https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html
- `IntoIterator`를 구현하면 "이 유형을 어떻게 반복자로 바꿀 수 있습니까?"라는 질문에 대답하게 됩니다.
  - https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html

> `Intolter` 유형에 대한 특성 경계입니다. `Intolter`는 Iterator 특성을 자체적으로 구현해야 함을 지정합니다. 꺾쇠 괄호 안의 항목은 반복자가 반환할 요소의 유형입니다.
> `IntoIterator` 특성을 구현하는 모든 유형은 `Iterator` 특성을 구현하는 연관된 유형을 제공해야 합니다. 이 연관된 유형은 `into_iter()` 메소드에서 반환되는 것입니다.