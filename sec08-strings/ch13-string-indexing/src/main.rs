/* String indexing*/

// https://doc.rust-lang.org/stable/std/str/struct.Chars.html
fn main() {
    // https://doc.rust-lang.org/stable/std/option/enum.Option.html
    let s1 = String::from("hello+∞+😊+ರ");

    let mut ch = s1.chars().next();
    if let Some(c) = ch {
        println!("{}", c)
    }

    /* Iterator의 'n번째' 메소드를 사용한 인덱싱 */
    ch = s1.chars().nth(6);
    if let Some(c) = ch {
        println!("{}", c);
    }

    /* 100번째 인덱스는 사용할 수 없으므로 nth(100)은 'None'을 반환한다 */
    ch = s1.chars().nth(100);
    if let Some(c) = ch {
        println!("{}", c);
    } else {
        println!("No character found");
    }

    let slice = &s1[6..=8];
    // ASCII 문자가 포함되어 있으면 범위를 벗어나지 않는 한 모든 Slice가 작동함
    // let slice = &s1[7..=10]; // Error ASCII가 아닌 문자가 포함된 문자열을 Slice 할 때 주의해야 함
    println!("{}", slice);

    /* Displaying byte representation of a unicode String */
    let message = String::from("hello+∞+😊+ರ");
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#method.as_bytes
    let byte_slice = message.as_bytes();
    for byte in byte_slice {
        print!("{:#X}\t", byte);
    }

    // https://doc.rust-lang.org/stable/std/string/struct.String.html#method.into_bytes
    let byte_array = message.into_bytes();
    println!("{:?}", byte_array);
    println!("{}", message);

}