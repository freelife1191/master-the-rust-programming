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
    for s in &mut words {
        if s == "hello" {
            s.push_str(" good morning");
        }
        println!("{s}");
    }

    //iterate by value
    // here, for loop consumes the array 'words'. i.e array 'words' is moved(It doesn't implement the copy trait)
    for s in words {
        println!("{s}");
    }

    //cannot access moved array
    //println!("{:?}", words); //Error

    /*
      원본 배열 값 변경 예시
     */
    let mut numbers = [ 23, 34, 5, 6, 7, 88 ];

    // 가변 참조 형으로 반복하면서 역참조 하여 원본 배열값 변경 처리
    /*
    for n in &mut numbers {
        if *n < 10 {
            *n = 0;
        }
    }
    */
    // 범위 표현식으로 가변형 numbers 배열을 순회하면서 원본 배열값 변경 처리
    for i in 0..numbers.len() {
        if numbers[i] < 10 {
            numbers[i] = 0;
        }
    }

    println!("{:?}", numbers);
}
