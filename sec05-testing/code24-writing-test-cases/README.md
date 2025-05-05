## Setting up and organizing test cases

```rust
#[cfg(test)]
mod tests {
    //test cases go here
}
```

1. `#[cfg(test)]`는 프로덕션용으로 빌드할 때가 아니라 테스트용으로 코드를 컴파일할 때만 주석이 달린 모듈을 포함하도록 Rust 컴파일러에 지시하는 조건부 컴파일 속성입니다
2. `mod 테스트 {}`는 테스트라는 모듈을 선언합니다. Rust에서 모듈은 코드를 구성하고 캡슐화하는 방법입니다

- https://doc.rust-lang.org/reference/attributes.html
- https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
- https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute

## Writing test cases

이 속성은 Cargo 빌드 시스템이 이 기능을 테스트 사례로 처리하도록 지시합니다.

즉, `cargo test` 명령을 실행할 때만 함수가 컴파일되고 실행되지만 최종 애플리케이션 바이너리에는 포함되지 않습니다

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_your_test_case_1() {
        
    }
    
    #[test]
    fn test_your_test_case_2() {
        
    }
    
    #[test]
    #[ignore]
    fn test_your_test_case_3() {
        
    }
}
```


이 속성은 Cargo 빌드 시스템이 이 기능을 테스트 사례로 처리하도록 지시합니다

즉, `cargo test` 명령을 실행할 때만 함수가 컴파일되고 실행되지만 최종 애플리케이션 바이너리에는 포함되지 않습니다


```rust
#[cfg(test)]
mod tests {
    // 이 함수는 테스트 케이스가 아닙니다
    // 테스트 케이스에서 사용되는 도우미 함수입니다
    fn helper_function() {
        
    }
    
    #[test]
    fn test_your_test_case_1() {
        
    }
    
    #[test]
    fn test_your_test_case_2() {
        
    }
    
    // more test cases
}
```

```rust
#[cfg(test)]
mod tests {
    // keep all the test cases to unit test convert_seconds_to_24hr_format
    // 1. when total_seconds is zero, function must return 00:00:00
    fn test_when_total_seconds_is_zero_expected_00_00_00() {
        // test code here
    }

    // 2. when total_seconds is 86400, function must panic
    fn test_when_total_seconds_is_86400_expected_panic() {
        // test code here
    }

    // 3. when total_seconds is 86399, function must return 23:59:59
    fn test_when_total_seconds_is_86399_expected_23_59_59() {
        // test code here
    }

    // 4. when total_seconds is negative, function must panic
    fn test_when_total_seconds_is_negative_expected_panic() {
        // test code here
    }
}
```