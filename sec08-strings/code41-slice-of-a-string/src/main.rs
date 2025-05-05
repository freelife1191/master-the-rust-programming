// str vs slice
// https://doc.rust-lang.org/std/primitive.str.html
fn main() {
    // slice of a string
    let s1 = String::from("hello");
    // 's2' is a slice of type &str
    let s2: &str = &s1[0..=3];
    println!("{}", s2); // output: hell

    // A String literal is actually a string slice
    // s3 is a string slice. its type is &str
    // s3의 유형은 실제로 문자열 슬라이스, s3가 스택에 있어야함
    // Stack에는 S3의 주소값이 저장되고 ROM 힙에는 실제 데이터가 저장됨
    let s3 = "hello!";

    /*
    let greeting: String = "Hello, World!".to_string();
    let hello = &greeting[0..5];
    println!("{}", hello); // Output: Hello

    let numbers = [1, 2, 3, 4, 5];
    let slice_of_numbers = &numbers[1..4];
    println!("{:?}", slice_of_numbers); // Output: [2, 3, 4]
    */

    // 문자열 슬라이스를 나타내는 str 이고, 또 다른 기본 유형은 문자열을 나타내는 슬라이스이다
    // 문자열 리터럴은 실제로 문자열 조각이다
    // str 형태로 참조한 데이터는 변경할 수 없음
    let greeting: String = "Hello, World!+∞".to_string();
    let hello: &str = &greeting[0..5];
    // let hello: &str = &mut greeting[0..5];
    // hello[0] = b'h'; // Error: cannot assign to `hello[..]` because it is borrowed
    println!("{}", hello); // Output: Hello

    // slice 형태로 가져온 데이터는 변경가능
    let mut numbers = [1, 2, 3, 4, 5];
    let slice_of_numbers: &mut [i32] = &mut numbers[1..4];
    slice_of_numbers[0] = 99;
    println!("{:?}", slice_of_numbers); // Output: [2, 3, 4]
}
