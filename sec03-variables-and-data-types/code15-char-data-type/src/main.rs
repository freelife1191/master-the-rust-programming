// https://en.wikipedia.org/wiki/List_of_Unicode_characters
fn main() {
    let infinity_symbol = '\u{221E}';
    println!("symbol = {}, usv = {}", infinity_symbol, infinity_symbol as u32);

    let tm = '®';
    println!("symbol = {}, usv = {}", tm, tm as i32);
    // 16진수로 출력
    println!("symbol = {}, usv = {:X}", tm, tm as i32);

    // About '\u{}' syntax
    let tm = '\u{00AE}';
    println!("symbol = {}, usv = {}", tm, tm as i32);

    let ka_in_kannada = '\u{C95}'; // let ka_in_kannada = 'ಕ';
    let ka_in_hindi = '\u{915}';
    let ka_in_chinese = '\u{5f00}';
    println!("{}\n{}\n{}", ka_in_kannada, ka_in_hindi, ka_in_chinese);

    // Converting u32 to char
    let usv_of_inf = 0x221e_u32;
    // println!("symbol = {}, usv = {}", usv_of_inf as char, usv_of_inf); // Error
    if let Some(inf_symbol) = std::char::from_u32(usv_of_inf) {
        println!("symbol = {}, usv = {}", inf_symbol, usv_of_inf);
    } else {
        println!("Not a valid Unicode scalar value");
    }

    // bool
    let is_raining = true;
    if is_raining {
        println!("Bring an umbrella!");
    } else {
        println!("Enjoy the sunshine!");
    }
}