# Implementing Trait `IntoIterator` for a custom type (Part 1)

## Note: Iterator vs IntoIterator

- `Iterator`를 구현할 때 "이 유형은 어떻게 시퀀스에서 값을 생성합니까?"라는 질문에 답하게 됩니다.
- `IntoIterator`를 구현하면 "이 유형을 어떻게 반복자로 바꿀 수 있습니까?"라는 질문에 대답하게 됩니다.

### IntoIterator

https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html

> `IntoIterator` 특성을 구현하는 모든 유형은 `Iterator` 특성을 구현하는 관련 유형을 제공해야 합니다.  
> 이 연관된 유형은 `into_iter()` 메소드에서 반환되는 것입니다.


## Implementing `IntoIterator` trait for a Custom Type

사용자 정의 유형에 대한 `IntoIterator` 특성을 구현하는 것은 여러 시나리오에서 특히 유용합니다:

1. **for 루프에서 직접 사용:** 사용자 정의 유형을 for 루프에서 직접 반복할 수 있어 유용성이 향상됩니다.
2. **반복의 유연성:** 값, 불변 참조 또는 가변 참조별로 요소를 반복할 수 있는 유연성을 제공합니다.
3. **반복자 메서드와의 호환성:** `map`, `filter` 및 `fold`와 같은 표준 Rust 반복자 메서드와 호환되는 유형을 만듭니다.
4. **Rust 패턴과의 일관성:** 사용자 정의 유형을 확립된 Rust 관용구에 맞춰 Rust 개발자에게 더욱 직관적으로 만듭니다.

## 사용자 정의 유형에 대해 `IntoIterator` 특성을 구현하는 방법은 무엇입니까?

- `IntoIterator` 특성을 사용자 정의 유형과 함께 효과적으로 사용하려면 반복자가 필요합니다.  
  해당 유형에 대한 반복자를 아직 사용할 수 없는 경우 먼저 `Iterator` 특성을 구현해야 합니다.  
  여기에는 해당 유형의 요소를 순차적으로 생성하는 방법을 정의하는 `next` 메소드를 사용하여 사용자 정의 반복자를 생성하는 작업이 포함됩니다.
- 반복자가 있으면 해당 유형에 대한 `IntoIterator` 특성을 구현할 수 있습니다.  
  이 구현에서는 유형을 자신이 만든 사용자 정의 반복자로 변환하는 방법을 지정합니다.  
  이를 통해 사용자 정의 유형을 `for` 루프 및 Rust의 다른 `iterator-based` 기능과 원활하게 사용할 수 있습니다.


## Exercise: 가격대를 기준으로 'Car' 컬렉션 반복

- `CarCollection`이라는 사용자 정의 데이터 유형을 갖춘 프로그램을 작성합니다.  
  이 컬렉션은 제조사, 모델, 가격과 같은 속성을 각각 포함하는 `Car` 객체 목록을 저장합니다.
- 핵심 요구 사항은 `CarCollection`에 대한 `IntoIterator` 특성을 세 가지 형식(값 기준, 불변 참조 기준, 가변 참조 기준)으로 구현하는 것입니다.
- 그러나 독특한 변형이 있습니다.  
  반복자의 `next` 메서드는 지정된 가격 범위 내의 자동차에 대해 반복되어야 합니다.  
  이는 `CarCollectio`n이 반복될 때 가격이 특정 범위 내에 있는 자동차(예: 20,000~50,000 USD 사이의 가격이 책정된 자동차)만 산출해야 함을 의미합니다.
