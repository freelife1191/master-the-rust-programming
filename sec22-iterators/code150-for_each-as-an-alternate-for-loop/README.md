# **``for`` 루프 대신 ``for_each`` 를 사용할 수 있는가?**

``for_each`` 는 ``for`` 루프의 대안으로 사용될 수 있으며, 특히 컬렉션의 각 아이템에 대해 작업을 수행하고 루프 내에서 어떤 값도 반환할 필요가 없을 때 유용함

## 부수 효과 수행 (Performing Side Effects)

각 아이템을 출력하거나 (가변 참조로 반복할 때) 각 아이템을 제자리에서 수정하는 것과 같은 부수 효과(side effect)를 수행하는 경우, ``for_each`` 가 더 나은 대안이 될 수 있음

```rust
let mut numbers = vec![1, 2, 3];

// Using a for loop
// for 루프 사용
for number in &mut numbers {
    *number *= 2;
}

// Using for_each
// for_each 사용
numbers.iter_mut().for_each(|number| *number *= 2);
```

## 이터레이터 어댑터 체이닝 시 편리함

``for_each`` 는 이터레이터 어댑터를 체이닝(chaining)할 때 더 편리할 수 있는데, 작업을 단일하고 흐르는 듯한 체인 안에 유지할 수 있게 해주기 때문임

```rust
let numbers = vec![1, 2, 3];

// Using a for loop
// for 루프 사용
for number in numbers.iter().map(|&n| n * 2).filter(|&n| n > 5) {
    println!("{}", number);
}

// Using for_each
// for_each 사용
numbers.iter().map(|&n| n * 2).filter(|&n| n > 5).for_each(|number| println!("{}", number));
```

## ``for`` 루프가 더 적절하거나 유일한 옵션일 수 있는 경우

1.  **``break`` 또는 ``continue`` 필요성**: 특정 조건에서 루프를 일찍 중단하거나 반복을 건너뛰어야 하는 경우, ``for_each`` 는 항상 모든 아이템을 반복하기 때문에 ``for`` 루프가 필요함
2.  **복잡한 루핑 로직**: 여러 다른 연산, 조건문, 또는 중첩 루프까지 포함하는 복잡한 루핑 로직이 있는 경우, 모든 로직을 ``for_each`` 와 함께 단일 클로저에 맞추려고 시도하는 것보다 전통적인 ``for`` 루프를 사용하는 것이 더 명확하고 유지보수하기 좋음
3.  **반환 값**: 루프 내에서 값을 반환해야 하는 경우 (예: 아이템을 검색하고 찾으면 즉시 반환), ``for`` 루프는 ``return`` 을 사용하여 루프와 감싸고 있는 함수를 즉시 종료할 수 있게 함 ``for_each`` 를 사용하면, 클로저 외부의 가변 참조에 값을 설정하는 것과 같은 다른 기술을 사용해야 함

```rust
// Using a for loop for complex logic
for item in collection {
    if some_condition(item) {
        perform_action(item);
    } else if other_condition(item) {
        perform_other_action(item);
    }
    // More complex operations...
}
```

여러 다른 연산, 조건문, 또는 중첩 루프까지 포함하는 복잡한 루핑 로직이 있는 경우, 모든 로직을 ``for_each`` 와 함께 단일 클로저에 맞추려고 시도하는 것보다 전통적인 ``for`` 루프를 사용하는 것이 더 명확하고 유지보수하기 좋음