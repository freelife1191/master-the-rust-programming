use std::io;
//We are using methods(like flush()) from the 'Write' trait of the std::io module
use std::io::Write;

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parse_hex_to_u32(hex_str: &str) -> u32 {
    let clean_hex_str = hex_str.trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(clean_hex_str, 16).expect("Failed to parse hex")
}

fn rgba8888_to_rgb565(rgba: u32) -> u16 {
    // Extract red, green, and blue components
    let red = (rgba >> 24) & 0xFF;
    let green = (rgba >> 16) & 0xFF;
    let blue = (rgba >> 8) & 0xFF;

    // Convert to RGB565
    let red_565 = (red >> 3) << 11;
    let green_565 = (green >> 2) << 5;
    let blue_565 = blue >> 3;

    (red_565 | green_565 | blue_565) as u16
}

fn main() {
    print!("Enter RGBA8888 data in hex format:");
    //표준 출력의 버퍼를 플러시합니다
    io::stdout().flush().expect("Failed to flush stdout");

    let input = read_user_input();
    let rgba888 = parse_hex_to_u32(&input);
    let rgb565 = rgba8888_to_rgb565(rgba888);
    println!("{:#x} in rgb565 format is {:#x}", rgba888, rgb565);
}
