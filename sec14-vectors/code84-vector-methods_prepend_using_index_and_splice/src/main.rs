#[derive(PartialEq, PartialOrd)] // PartialEq: 비교 연산자를 사용할 수 있도록 함, PartialOrd: 부분적으로 순서를 정의하는 특성
struct Person {
    name: String,
    age: u32,
}

fn main() {
    /*
    Is comparing vectors possible?
    */
    let vec1 = vec![1, 2];
    let vec2 = vec![1, 2, 3];

    if vec1 < vec2 {
        println!("vec1 is smaller");
    } else {
        println!("vec2 is smaller");
    }


    let person1 = Person { name: String::from("Alice"), age: 30 };
    let person2 = Person { name: String::from("Bob"), age: 35 };

    let vec1 = vec![person1];
    let vec2 = vec![person2];

    println!("Are the vectors equal? {}", vec1 == vec2);
    // println!("Are the vectors equal? {}", vec1 < vec2);

    /*
    Prepend

    insert()
    */
    let mut vec = vec![1, 2, 3];
    vec.insert(0, 25);
    println!("{:?}", vec); // prints: [25, 1, 2, 3]

    /*
    Prepend using splice
    */
    let mut main_vec = vec![4, 5, 6];
    let vec_to_prepend = vec![1, 2, 3];

    main_vec.splice(0..0, vec_to_prepend);

    println!("{:?}", main_vec); // prints: [1, 2, 3, 4, 5, 6]
}
