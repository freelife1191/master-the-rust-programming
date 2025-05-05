fn main() {
    // let numbers = [1, 2, 3, -4, 5]; // Invalid Array
    let numbers = [1, 2, 3, -4, 5]; // Valid Array

    // 레이블 지정 루프
    let message = 'outer: loop {
        for n in numbers {
            if n < 0 {
                break 'outer "Invalid Array";
            }
        }
        break 'outer "Valid Array";
    };

    println!("{message}");
}
