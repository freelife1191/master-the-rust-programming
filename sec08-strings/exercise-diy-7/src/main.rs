use std::io;

fn main() {
    let (passwd, re_entered_passwd) = promt_passwd_change_message();

    println!("{}", print_status(
        check_password_strength(&passwd),
        is_password_match(&passwd, &re_entered_passwd),
    ));
}

fn promt_passwd_change_message() -> (String, String) {
    println!(
        r##"
###### 새 비밀번호 설정 ######
1. [a-z] 사이에 문자 1개 이상
2. [0~9] 사이의 숫자 1개 이상
3. [A-Z] 사이에 최소 1개의 문자
4. [$#@]에서 1자 이상
5. 거래 비밀번호의 최소 길이: 6
6. 거래 비밀번호 최대 길이: 12
        "##
    );

    println!(
        "새 비밀번호를 입력하십시오:"
    );
    let mut passwd = String::new();
    io::stdin().read_line(&mut passwd).expect("stdin에서 읽는 중 오류가 발생");

    let mut re_enter_passwd = String::new();
    println!(
        "새 비밀번호를 다시 입력하세요:"
    );
    io::stdin().read_line(&mut re_enter_passwd).expect("stdin에서 읽는 중 오류가 발생");

    (passwd.trim().to_string(), re_enter_passwd.trim().to_string())
}


fn is_password_match(passwd_1: &str, passwd_2: &str) -> bool {
    passwd_1 == passwd_2
}

fn is_password_contain_lower_case_letters(passwd: &str) -> bool {
    for ch in passwd.chars() {
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_lowercase
        if ch.is_ascii_lowercase() {
            return true;
        }
    }

    false
}

fn is_password_contain_upper_case_letters(passwd: &str) -> bool {
    for ch in passwd.chars() {
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_uppercase
        if ch.is_ascii_uppercase() {
            return true;
        }
    }

    false
}

fn is_password_contains_digits(passwd: &str) -> bool {
    for c in passwd.chars() {
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_digit
        if c.is_ascii_digit() {
            return true;
        }
    }

    false
}

fn is_allowed_special_chars(c: char) -> bool {
    c == '$' || c == '@' || c == '#'
}

fn is_password_contains_allowed_special_chars(passwd: &str) -> bool {
    for c in passwd.chars() {
        if is_allowed_special_chars(c) {
            return true;
        }
    }

    false
}


fn is_password_contains_illegal_special_chars(passwd: &str) -> bool {
    for c in passwd.chars() {
        // https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_alphanumeric
        if c.is_ascii_alphanumeric() || is_allowed_special_chars(c) {
            continue;
        } else {
            return true;
        }
    }

    false
}

fn is_password_length_valid(passwd: &str) -> bool {
    (passwd.len() >= 6) && (passwd.len() <= 12)
}

fn check_password_strength(passwd: &str) -> i32 {
    //weak = 0, strong = 1
    if is_password_contain_lower_case_letters(passwd) &&
        is_password_contain_upper_case_letters(passwd) &&
        is_password_contains_digits(passwd) &&
        is_password_contains_allowed_special_chars(passwd) &&
        !is_password_contains_illegal_special_chars(passwd) &&
        is_password_length_valid(passwd) {
        1
    } else {
        0
    }
}

// Rust 컴파일러에게 반환하는 문자열 슬라이스를 설명함
// 정적 수명을 가지며 컴파일러 반환 값을 관리
// static을 추가하면 코드가 통과하는지 여부를 확인할 수 있음
fn print_status(weak_or_strong: i32, match_status: bool) -> &'static str {
    if !match_status {
        "비밀번호가 일치하지 않습니다.\n비밀번호 변경에 실패했습니다"
    } else if weak_or_strong == 1 {
        "비밀번호가 강력합니다.\n비밀번호 변경에 성공했습니다"
    } else {
        "비밀번호가 약함\n비밀번호 변경에 실패했습니다"
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_when_both_the_passwords_are_same_fn_returns_true() {
        assert_eq!(true, is_password_match("abc", "abc"));
    }

    #[test]
    fn test_when_both_the_passwords_are_not_same_fn_returns_false() {
        assert_eq!(false, is_password_match("abc", "abcd"));
    }

    mod passwd_strength {
        use crate::*;

        #[test]
        fn test_when_password_is_empty_string_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength(""));
        }

        #[test]
        fn test_when_password_contains_only_a_to_z_letters_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("abc"));
        }

        #[test]
        fn test_when_password_contains_only_A_to_Z_letters_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("ABC"));
        }

        #[test]
        fn test_when_password_contains_only_digits_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("123"));
        }

        #[test]
        fn test_when_password_length_is_less_than_6_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("123"));
        }

        #[test]
        fn test_when_password_length_is_greater_than_12_fn_retuns_weak() {
            //weak = 0, strong = 1
            assert_eq!(0, check_password_strength("abcdEF@123456789"));
        }

        #[test]
        fn test_when_password_contains_lower_case_letters_fn_returns_true() {
            assert_eq!(true, is_password_contain_lower_case_letters("abc"));
        }

        #[test]
        fn test_when_password_does_not_contain_lower_case_letters_fn_returns_false() {
            assert_eq!(false, is_password_contain_lower_case_letters("ABC123"));
        }

        #[test]
        fn test_when_password_contains_any_upper_case_letters_fun_returns_true() {
            assert_eq!(true, is_password_contain_upper_case_letters("12345A"));
        }

        #[test]
        fn test_when_password_contains_no_upper_case_letters_fun_returns_false() {
            assert_eq!(false, is_password_contain_upper_case_letters("12345abc###"));
        }

        #[test]
        fn test_when_password_contains_illegal_special_chars_fun_retuns_true() {
            assert_eq!(true, is_password_contains_illegal_special_chars("12345abc###_"));
        }

        #[test]
        fn test_when_password_does_not_contain_illegal_special_chars_fun_retuns_false() {
            assert_eq!(false, is_password_contains_illegal_special_chars("12345abc###"));
        }

        #[test]
        fn test_when_passwords_match_and_it_is_strong_print_strong_and_successful() {
            assert_eq!(
                "비밀번호가 강력합니다.\n비밀번호 변경에 성공했습니다",
                print_status(1, true)
            );
        }

        #[test]
        fn test_when_weak_passwords_match_print_weak_and_un_successful() {
            assert_eq!(
                "비밀번호가 약함\n비밀번호 변경에 실패했습니다",
                print_status(0, true)
            );
        }
    }
}













































