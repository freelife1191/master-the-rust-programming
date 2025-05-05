fn main() {
    // S1은 "hello" 문자열의 소유자이다
    let s1 = String::from("hello");

    // 's1'의 소유권이 's'로 이전된다
    // 'fun1'은 's'의 소유권을 s2로 다시 반환한다
    let s2 = fun1(s1);

    /*
      배열 요소 소유권 이전
     */
    let array1 = [5, 6, 7, 8]; //[i32; 4]

    let array2: [String; 3] = [
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ];

    let item = array1[1];
    println!("{}", item); // 복사된 값이 출력된다
    // Error 배열 요소 중 하나의 소유권 이전을 허용하지 않음
    // 배열 요소중 하나의 소유권이 이전 돼 유효하지 않아지면 프로그램 버그가 발생할 수 있음
    // 문바열 배열이나 배열과 같은 힙 기반 데이터 구조를 인덱싱할 때 문자열이나 벡터가 포함된 구조의 경우 대신 요소를 빌려 색인화 할 수 있음
    // let str_item = array2[1];
    // let str_item: &str = &array2[1]; // 슬라이스 형식으로 변환
    // println!("{}", str_item);

    // 배열 요소의 소유권이 이전되므로 루프가 끝나면 배열 요소들은 유효하지 않게됨
    /*
    for str in array2 {
        println!("{}", str);
    }
    println!("{:?}", array2); // 배열 요소의 소유권이 str로 이동되어 출력되지 않음
    */

    /*
    `&array`를 사용하면 루프가 빌려서 배열을 반복하도록 지시합니다
    이렇게 하면 루프가 배열 요소를 이동하려고 시도하지 않습니다
    이 접근 방식은 배열 요소에 대한 읽기 전용 액세스만 필요할 때 유용합니다
     */
    // 모든 배열 요소의 소유권을 이동하지 않고 빌려서 출력
    for str in &array2 {
        println!("{}", str);
        // println!("{}", *str); // 빌려서 출력하므로 * 연산자도 사용가능
    }
    println!("{:?}", array2); // 배열 요소의 소유권이 이동되지 않아 출력됨

    /*
      &mut T는 이동이고 &T는 복사이다
     */
    // let msg = "Hello".to_string();
    // let ref1 = &msg; // 불변 참조는 읽기 전용
    // let ref2 = ref1; // 참조자는 복사됨
    let mut msg = "Hello".to_string(); // mut 키워드를 사용하면 어떤 유형이든 될 수 있음
    let ref1 = &mut msg; // 가변 참조는 읽기/쓰기 가능
    let ref2 = ref1; // 참조1이 참조2로 이동되어 참조1은 더 이상 사용할 수 없음
    // 이 부분을 복사로 취급한다면 다른 결과의 여러 변경 가능한 참조가 있을 수 있어 문제가 발생할 수 있음

    // Error: 참조1이 참조2로 이동되었으므로 사용할 수 없어 컴파일 오류가 발생함
    if ref1 == ref2 {
        println!("Both refs are pointing to the same data")
    }
}

fn fun1(mut s: String) -> String {
    println!("{}", s);
    s.push_str(" world");
    s // 소유권을 반환한다
}
