fn main() {
    example_function();
}

/**
    변수가 범위를 벗어날 때 C와 Rust의 메모리 정리 비교
**/
fn example_function() {
    let s = String::from("Hello");
    // send_data(s);
    /* 's'가 범위를 벗어나면 자동으로 정리가 발생합니다 */
}
