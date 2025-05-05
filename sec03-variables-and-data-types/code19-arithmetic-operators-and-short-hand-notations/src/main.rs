fn main() {
    // +, -. /, *, %
    let arg1: u8 = 5;
    let arg2: i32 = 30;

    // let sum = arg1 / arg2; // Error
    let arg1 = 5;
    let arg2 = 30;
    let sum = arg1 % arg2;
    println!("{} % {} = {}", arg1, arg2, sum);

    let patterns = [1, 0, 0, 1, 0, 1, 1, 1];
    let mut i = 0;
    loop {
        // control_light(patterns[i % patterns.len()]);
        // delay(1000);
        // i = i + 1;
    }

    // 산술 연산을 위한 약식 표기법
    // +=, -=, *=, /=, %=
    let mut a = 5;
    // a = a * 5;
    a *= 5;
}
