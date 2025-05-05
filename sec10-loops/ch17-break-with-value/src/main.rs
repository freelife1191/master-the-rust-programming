/*
 Example of 'break with value'.
 i가 3이면 i * 2 값으로 'break' 문이 실행됩니다.
 그러면 루프가 종료되고 루프 결과로 값 6이 반환됩니다.
 */
fn main() {
    let mut i = 0;

    let result = loop {
        if i == 3 {
            break i * 2;
        }
        i += 1;
    };

    println!("result = {}", result);
}

/*
  break with return value
 */
fn find_index_of_value(numbers: [i32; 10], value: i32) -> Option<usize> {
    let mut index = 0;

    loop {
        if index >= numbers.len() {
            return None;
        }
        if numbers[index] % 2 == 0 {
            return Some(index);
        }
        index += 1;
    }
}