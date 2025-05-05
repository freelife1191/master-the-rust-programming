fn main() {
    // https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
    // let mut s1 = String::new();
    // s1.push_str("Good");
    let s1 = String::from("Good"); // 변경될 필요가 없어 mut 제거
    let s2 = String::from(" Morning");
    // let s3 = s1 + s2; // Error String + String
    // https://doc.rust-lang.org/std/string/struct.String.html#impl-Add%3C%26str%3E-for-String
    // let s3 = s1 + &s2;
    // println!("{}", s3);
    let s3 = format!("{}{}", s1, " evening"); // s3가 각각의 문자열을 복제해서 소유
    println!("{}", s3);
    println!("{}", s1); // s1에 대한 힙 메모리는 그대로 유지됨

    // let s4 = "Good" + "Night"; // Error: &str + &str
}
