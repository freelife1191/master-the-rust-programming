/*
    예제는 Rust의 다양한 반복 방법을 보여준다
    불변 빌림, 가변 빌림 및 가치 기준
*/
fn main() {
    let mut words = [
        "hello".to_string(),
        "World".to_string(),
        "how".to_string(),
        "are".to_string(),
        "you".to_string(),
    ];

    //iterate by immutable borrrow (read only iteration)
    for s in &words {
        println!("{s}");
    }

    //iterate by mutable borrow (you can modify the elements)
    // type of 'str' is &mut String
    for s in &mut words {
        if s == "hello" {
            /*
            여기서 `str.push_str()`은 '그냥 작동'합니다
            왜냐하면 Rust는 `push_str()`과 같은 String 메서드를 호출할 때 필요에 따라 여러 참조 레이어를 통해 `str`을 자동으로 역참조하기 때문입니다
             */
            // (*s).push_str(" good morning");
            s.push_str(" good morning");
        }
        // println!("{:?}", words);
        println!("{s}");
    }

    //iterate by value
    // here, for loop consumes the array 'words'. i.e array 'words' is moved(It doesn't implement the copy trait)
    for s in words {
        println!("{s}");
    }

    //cannot access moved array
    //println!("{:?}", words); //Error

}
