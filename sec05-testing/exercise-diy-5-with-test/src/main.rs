use std::io;

/*
    Convert seconds to HH:MM:SS format
*/

fn main() {

    let total_seconds = get_user_input();
    println!(
        "Time in 24hr format is: {}",
        convert_seconds_to_24hr_format(total_seconds)
    );
}


fn convert_seconds_to_24hr_format(total_seconds: u32) -> String {
    if total_seconds > 86399 {
        panic!("Your input should be between 0 to 86,399 ");
    }

    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    let format_24hr = format!(
        "{:02}:{:02}:{:02}",
        hours, minutes, seconds
    );

    return format_24hr; //String
}


fn read_from_stdin() -> String {
    let mut input = String::new();

    println!("Enter the time of the day in seconds(0 to 86,399):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn parse_string_as_u32(input: String) -> u32 {
    let total_seconds: u32 = input
        .trim()
        .parse()
        .expect("Input number only without any sign!");

    return total_seconds;
}


fn get_user_input() -> u32 {

    return parse_string_as_u32(read_from_stdin());
}

#[cfg(test)]
mod tests {

    mod time_format {
        //모든 테스트 케이스를 단위 테스트로 유지합니다.
        //1. total_seconds가 0이면 함수는 00:00:00을 반환해야 합니다.
        #[test]
        fn test_when_total_seconds_is_zero_expected_00_00_00() {
            assert_eq!("00:00:00", super::super::convert_seconds_to_24hr_format(0));
        }

        //2. total_seconds가 86400이면 함수는 패닉 상태가 되어야 합니다.
        #[test]
        #[should_panic(expected = "should be between 0 to 86,399")]
        fn test_when_total_seconds_is_86400_function_must_panic() {
            super::super::convert_seconds_to_24hr_format(86400);
        }

        //3. total_seconds가 86399이면 함수는 23::59::59를 반환해야 합니다.

        //4. when total_seconds is -ve, function must panic
        #[test]
        #[should_panic]
        fn test_when_total_seconds_is_negative_function_must_panic() {
            // super::super::convert_seconds_to_24hr_format(-30000); // Error
        }

    }

    mod parse_user_input {
        //모든 테스트 케이스를 단위 테스트로 유지 get_user_input
        //1) 사용자가 0을 입력하면 함수는 0을 반환해야 합니다.
        #[test]
        fn test_user_enters_zero_function_must_return_zero() {
            assert_eq!(0, super::super::parse_string_as_u32("0\n\r".to_string()));
        }
        //2) 사용자가 정수를 입력하면 functin은 정수를 반환해야 합니다.
        //3) 사용자가 음수를 입력하면 함수는 패닉 상태가 됩니다.
        //4) 사용자가 10진수를 입력하면 함수는 패닉 상태가 되어야 합니다.
        //5) 사용자가 숫자 이외의 문자를 입력하면 함수가 패닉 상태가 됩니다.
        //6) 사용자가 아무것도 입력하지 않으면(Enter 키만 누르면) 함수는 패닉 상태가 됩니다.
        //7) 사용자가 u32의 최대값보다 큰 숫자를 입력하면 함수는 패닉 상태가 됩니다.
        #[test]
        // #[ignore] // Test Pass
        // cargo test --ignored // ignore 포함 모든 테스트 케이스 실행
        // cargo test -- --include-ignored // ignore 포함 모든 테스트 케이스 실행
        #[should_panic]
        fn test_user_enters_number_greater_than_max_u32_function_must_panic() {
            let some_big_number_as_string = "4294967296\n\r".to_string();
            super::super::parse_string_as_u32(some_big_number_as_string);
        }

    }

}