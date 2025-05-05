fn main() {
    // suffixes to specify the type of a number
    // For inregers, these suffixes can be used: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    // For floating point numbers, these suffixes can be used: f32, f64
    let num1 = 100_u8;
    let num2 = 0xfffu32;
    let num3 = 0.5_f32;

    // Underscore literal
    let x = 1_000_000; // x = 1000000
    let y = 0xff_ff_ff_ff_u32; // y = 0xffffffff
    let z = 3_.1423_56f32; // z = 3.142356

    // Storing ascii code
    let code = b'+';
    // let code = '+'_u8; // Invalid sysntax
    let ascii_code_of_plus = b'+';
    println!("{}", ascii_code_of_plus);
    let ascii_code_of_plus = 43;
    println!("{}", ascii_code_of_plus as u8 as char);
}
