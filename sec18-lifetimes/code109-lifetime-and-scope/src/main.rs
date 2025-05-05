/*
Rust가 수명을 분석하지 못하고 여러분의 도움이 필요한 시나리오를 생각해 보자
 */
// fn extend_string(original: &mut String, data: &str) -> &str {
fn extend_string<'a>(original: &'a mut String, data: &str) -> &'a str {
    original.push_str(data);
    original
}
fn main() {

    let mut original = String::from("hello");
    let data = String::from("world");
    let result = extend_string(&mut original, &data);

    println!("{}", result);

}
