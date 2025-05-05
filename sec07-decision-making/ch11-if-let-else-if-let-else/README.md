# if-else if-else

```
if expression1 {
    // 표현식1이 참인 경우 실행할 코드 블록
} else if expression2 {
    // 표현식1이 거짓이고 표현식2가 참인 경우 실행할 코드 블록
} else {
    // 모든 조건이 거짓인 경우 실행할 코드 블록
}
```

- `if/else if/else` 체인의 마지막 else 분기는 필수가 아닙니다.
- 일반적으로 `if/else if/else` 문에 코드가 없더라도 최종 `else` 블록을 포함하는 것이 좋습니다. 이렇게 하면 가능한 모든 사례가 다루어지고 코드가 더욱 명확해지고 이해하기 쉬워집니다


## if let

Syntax

```
if let pattern = expression {
    // 표현식이 패턴과 일치하면 실행할 코드
} else {
    // 표현식이 패턴과 일치하지 않는 경우 실행할 코드
}
```


> if let 문은 패턴이 하나만 있는 match 문의 약어입니다