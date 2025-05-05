fn main() {
    // 메모리 표현
    let value = 42;
    let ref_of_value = &value;
    println!("Value is {}",*ref_of_value); // 수동 역참조
    println!("Value is {}",ref_of_value); // 자동 역참조
    println!("Value Memory Location {:p}",ref_of_value); // value 메모리 주소값
    println!("Value Memory Location {:p}",&ref_of_value); // ref_of_value 메모리 주소값
}
