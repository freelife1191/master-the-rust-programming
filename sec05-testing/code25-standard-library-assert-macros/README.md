# Rust assert macros


'Rust assert macros'는 테스트 실행 중 조건을 확인하고 코드가 예상대로 작동하는지 확인하는 데 도움이 됩니다

- `assert!(expression);`
- `assert_eq!(left, right);`
- `assert_ne!(left, right);`
- `debug_assert!(expression);`
- `#[should_panic(expected = "panic message")]`

https://doc.rust-lang.org/std/macro.assert.html


## assert!(expression)

- `assert!` 매크로는 주어진 조건을 확인하고 표현식이 false로 평가되면 프로그램이 패닉 상태가 되도록 합니다.
- 디버그 및 릴리스 빌드 모두에서 활성화됩니다. 즉, 프로그램이 어떻게 컴파일되든 `assert!`는 항상 검사를 수행하고 조건이 충족되지 않으면 패닉 상태가 됩니다.
- 사용 예시: https://doc.rust-lang.org/std/macro.assert.html


## assert_eq!(left, right);

- 이 매크로는 두 표현식이 동일한 값으로 평가되는지 확인하는 데 사용됩니다
- 함수의 출력이 예상 결과와 일치하는지 확인하기 위해 테스트에 일반적으로 사용됩니다
- 매크로는 일반적으로 왼쪽과 오른쪽이라고 하는 두 개의 인수를 사용하며 어느 것이 예상 값이고 어느 것이 실제 값인지는 중요하지 않습니다. 값이 동일하지 않으면 `assert_eq!`는 `panic`을 발생시킵니다


## `#[should_panic(expected = "panic message")]`

- 이 속성은 테스트에서 특정 코드 조각이 특정 패닉 메시지와 함께 패닉을 유발해야 한다고 주장하는 데 사용됩니다
- 이 속성으로 테스트 함수에 주석을 달면 테스트 중인 코드(CUT)가 패닉 상태가 될 것으로 예상되며 지정된 텍스트가 포함된 메시지와 함께 CUT가 실제로 패닉 상태인 경우에만 테스트가 통과해야 함을 나타냅니다

```rust
#[cfg(test)]
mod tests {
    // keep all the test cases to unit test convert_seconds_to_24hr_format
    // 1. when total_seconds is zero, function must return 00:00:00
    fn test_when_total_seconds_is_zero_expected_00_00_00() {
        assert_eq!("00:00:00", super::super::convert_seconds_to_24hr_format(0));
    }

    // 2. when total_seconds is 86400, function must panic
    #[test]
    #[should_panic(expected = "should be between 0 to 86,399")]
    fn test_when_total_seconds_is_86400_function_must_panic() {
        super::super::convert_seconds_to_24hr_format(86400);
    }

    // 3. when total_seconds is 86399, function must return 23:59:59
    fn test_when_total_seconds_is_86399_expected_23_59_59() {
        // test code here
    }

    // 4. when total_seconds is negative, function must panic
    /*
    #[test]
    #[should_panic]
    fn test_when_total_seconds_is_negative_function_must_panic() {
        super::super::convert_seconds_to_24hr_format(-30000);
    }
    */
}
```