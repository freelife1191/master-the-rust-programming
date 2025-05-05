fn main() {
    /**
       Bitwise operators in Rust
    **/
    // ! acts as Bitwise NOT
    let x = 0b0000_1111_u8; // In binary: 00001111
    let y = !x; // Results in 11110000 (inverted bits)
    println!("x={:b}, y={:b}", x, y); // 00001111, 11110000

    // ! acts as Logical NOT
    let a = true;
    let b = !a; // Results in false
    println!("a={}, b={}", a, b); // a=true, b=false

    /**
       Bitwise AND and Bitwise OR
    **/
    let a = 0xFF;
    let b = 0xF;
    let c = a & b;
    println!("{c}"); // 15
    // a b0000 0000 0000 0000 0000 0000 1111 1111
    // &
    // b b0000 0000 0000 0000 0000 0000 0000 1111
    // a & b = b0000 0000 0000 0000 0000 0000 0000 1111

    let a = 0x1A;
    let b = 0xF;
    let c = a | b;
    // a b0000 0000 0000 0000 0000 0000 0001 1010
    // |
    // b b0000 0000 0000 0000 0000 0000 0000 1111
    // a | b = b0000 0000 0000 0000 0000 0000 0000 1111

    /**
        Bitwise Left Shift (<<)
    **/
    let a = -2_i8; // 1111 1110
    let b = a << 1; // 1111 1100
    println!("{b}"); // -4

    /**
        Bitwise Right Shift (>>)
    **/
    let a = 0x80; // 1000 0000 (-128)
    let b = a >> 2; // 1110 0000 (-32)
    println!("a={a}, b={b}"); // -32

    // 숫자의 4~12번째 비트 위치 추출
    let num = 0x00ABCDEF;
    let mask = 0x1FF << 4; // 0000 0000 0000 0000 0001 1111 1111 0000
    let res = ((num & mask) >> 4) & 0x1FF;
    println!(",{:#X}", res); // 000001DE
}
