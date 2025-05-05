fn main() {
    // 항목에 대한 불변 참조를 가져와 반복합니다(빌림).
    /*
    let array = ["one".to_string(), "two".to_string()];

    for e in array.iter() {
        println!("{}", e);
    }
    */
    let mut array = [1, 2, 3, 4];

    for e in &mut array {
        *e += 1;
    }

    println!("{:?}", array);

}
