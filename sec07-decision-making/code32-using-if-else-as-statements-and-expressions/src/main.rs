fn main() {
    // if and else statement in Rust
    let age = 10;
    /*
    if age < 18 {
        println!("you cannot vote");
    } else {
        println!("you can vote");
    }
    */

    // Expression vs Statement
    // 블록의 값은 마지막 표현식의 값, 세미콜론은 사용하면 안됨
    let message = if age < 18 {
        println!(" this is if block");
        10
        // 10; // Error: Semicolon
    } else {
        println!(" this is else block");
        100
        // 100; // Error: Semicolon
    };

    println!("{}", message);
    // 반환할 유용한 값이 없음을 의미, 유용한 가치가 없거나 가치가 없음을 나타냄
    // println!("{:?}", message); // Unit Type
}