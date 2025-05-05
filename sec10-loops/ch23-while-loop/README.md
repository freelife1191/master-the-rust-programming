## while loop

Rust 의 `while` 루프는 조건이 참인 한 코드 블록의 반복 실행을 허용합니다

```
while condition {
    ...
}
```


Rust 의 `while let` 루프는 패턴이 값과 일치하는 한 코드 블록의 반복 실행을 허용합니다

```
while let pattern = value {
    ...
}
```


### When to use for?

- **Iterating over collections**: 컬렉션(배열, 벡터 또는 Iterator 특성을 구현하는 모든 항목)이 있고 각 요소에 대해 작업을 수행하려는 경우  
  for 루프는 컬렉션을 반복하는 가장 관용적이고 간단한 방법입니다

```
for element in collection {
    // perform operations on element
}
```

### When to use while loop?

- **Condition based loops**: 특정 조건이 충족되는 한 계속해서 루프를 실행하고 싶은데, 미리 반복 횟수를 결정할 수 없는 경우

```
while water_level_not_full == true {
    // keep pumping
}
```