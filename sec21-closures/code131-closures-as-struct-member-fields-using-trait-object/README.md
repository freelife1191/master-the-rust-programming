# Closures as struct member fields

클로저를 구조체에 저장하는 것은 여러 시나리오에서 유용할 수 있습니다:

1. 이벤트 처리
2. 사용자 정의 가능한 동작
3. 지연된 실행
4. 반복자 어댑터

## Methods to store a closure in a struct

1. As a trait object
2. As a generic struct

## As a trait object

`Box<dyn Fn()>`과 같은 특성 개체를 사용하여 구조체 필드에 클로저를 저장할 수 있습니다
`Box<dyn FnMut()>` 또는 `Box<dyn FnOnce()>`. 이렇게 하면 다음의 클로저를 저장할 수 있습니다
특정 특성을 구현하는 모든 유형입니다


```rust
struct MyStruct {
    val: i32,
    action: Box<dyn Fn(i32) -> i32>,
}
```

특성 이름 앞에 `dyn`을 사용하면 해당 특성을 특성 개체로 사용하겠다는 신호를 보내 해당 특성을 구현하는 모든 유형에 대해 동적 디스패치를 허용합니다

**Write** 특성을 구현하는 모든 유형을 나타낼 수 있기 때문에 컴파일 타임에는 크기를 알 수 없습니다

특성 개체를 사용할 때 다음과 같은 포인터 같은 유형 뒤에 있어야 합니다
- 참조: `&dyn Trait` 또는 `&mut dyn Trait`
- 상자: `Box<dyn Trait>`
- `Rc<dyn Trait>`, `Arc<dyn Trait>` 등과 같은 `Deref` 특성을 구현하는 기타 스마트 포인터 또는 사용자 정의 


`action` 은 `Fn(i32) -> i32` 특성에 대한 특성 개체를 포함하는 상자입니다  
이는 액션이 `Fn(i32) -> i32` 시그니처를 구현하는 모든 유형의 클로저(또는 다른 객체)를 보유할 수 있음을 의미합니다  
여기서 **Box** 를 사용하는 것은 특성 객체가 동적 크기를 가지므로 저장하기 위해 boxed (또는 다른 종류의 포인터 뒤에)해야 하기 때문에 필요합니다
