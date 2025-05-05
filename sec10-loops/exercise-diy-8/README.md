# Exercise-diy-8

**힌트**: 주어진 char 배열이 회문인지 확인하려면, 길이의 절반까지 배열을 반복한다  
루프가 반복될 때마다 배열 시작 부분의 현재 인덱스에 있는 문자와 배열 끝 부분의 해당 인덱스에 있는 문자를 비교한다  
일치하는 모든 문자가 동일하면 배열은 회문이다  

```rust
fn is_palindrome(chars: &[char]) -> bool {
    let len = chars.len();

    println!("chars: {:?}, len: {}, len / 2: {}", chars, len, len / 2);
    for i in 0..len / 2 {
        println!("len({}) - 1 - i({}) = {}, chars[{}]: {}, chars[{}]: {}", len, i, len - 1 - i, i, chars[i], len - 1 - i, chars[len - 1 - i]);
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let char_array_1 = ['r', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_1) {
        println!("주어진 배열은 회문입니다.");
    } else {
        println!("주어진 배열은 회문이 아닙니다");
    }

    let char_array_2 = ['b', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_2) {
        println!("주어진 배열은 회문입니다.");
    } else {
        println!("주어진 배열은 회문이 아닙니다");
    }
}

```