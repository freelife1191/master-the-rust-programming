/* Displaying byte representation of a unicode String */
fn main() {
    let message = String::from("hello+∞+😊+ರ"); // 유니코드 형식 보기

    /* 불변으로 문자열에 바이트로 접근하기 */
    // // https://doc.rust-lang.org/stable/std/string/struct.String.html#method.as_bytes
    let byte_slice: &[u8] = message.as_bytes();

    // 문자열이 메모리에 저장되는 형식
    for byte in byte_slice {
        print!("{}\t", byte);
    }

    // UTF-8로 인코딩된 문자열이 메모리에 저장되는 방식 보기
    for byte in byte_slice {
        print!("{:#X}\t", byte);
    }

    // 문자열의 바이트를 수정할 수 없음으로 인해 바이트 배열을 제공하는 방법 찾기
    println!("\r");

    /* 문자열을 바이트 벡터로 변환 */
    // https://doc.rust-lang.org/stable/std/string/struct.String.html#method.into_bytes
    let byte_array  = message.into_bytes();
    println!("\r{:?}", byte_array);
    // 문자열은 소비되어 바이트의 vec으로 변환되기 때문에 다시 사용할 수 없음
    //println!("{}", message); //Error 이미 이동된 값을 빌리려고 해서 에러 발생

}