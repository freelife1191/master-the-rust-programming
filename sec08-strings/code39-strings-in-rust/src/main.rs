fn main() {
    // type of 'greeting' is &str
    // the string literal "Hello World" cannot be modified
    // let greeting = "Hello World";
    // println!("{}", greeting);

    /*
        The String data type
    */
    // 새로운 변경 가능한 문자열 생성
    // 변수 'greeting'의 유형은 문자열입니다.
    let mut greeting = String::from("Good morning");

    // replace "morning" with "evening"
    greeting.replace_range(5.., "evening");

    // append ", world!" to the string
    greeting.push_str(", world!");

    println!("{}", greeting); // prints "Good evening, world!"

    /*
        문자열 리터럴을 사용하여 문자열 만들기
        문자열이 범위를 벗어날 때 힙 정리
     */
    let my_string = String::from("Hello, world!");
    // 문자열을 저장하기 위해 할당된 힙 메모리는 중괄호를 벗어나면 자동으로 할당 해제된다
    println!("{}", my_string);

    // 이는 'from' 메소드를 사용하는 것과 동일합니다.
    let my_string = "Hello, world!".to_string();

    /*
      to_string()
     */
    let mut num_as_string = 3.1416.to_string();
    num_as_string.insert_str(0, "PI = ");
    println!("{}", num_as_string); // Output: "PI = 3.1416"


    /*
      얕은 복사(Shallow copy)
     */
    // :: operator - 이중 콜론 연산자는 관련 함수 또는 정적 메서드에 액세스하는데 사용됨
    let s1 = String::from("Hello");
    let s2 = s1; // s1의 소유권이 s2로 이전됨 (힙 문자열의 전체 내용을 복사하지는 않고 소량의 데이터만 복사하기 때문에 빠른 작업이다)
    // println!("{}, world!", s1); // s2로 이전되었으므로 s1은 더 이상 사용할 수 없음
    println!("{}, world!", s2); // s2는 s1과 동일한 메모리를 가리킨다

    /*
      깊은 복사(Deep copy)
      문자열 값이 복제되면 원본과 동일한 내용으로 새 힙 할당이 생성된다
      모든 값에는 실제로 완전히 정확하지 않은 단일 소유자가 있다
      (In Rust every piece of data has a single owner at any given time)
      힙 기반 데이터의 복사본을 여러개 만드는 것은 매우 비싸다
     */
    /*
    let s1 = String::from("Hello");
    let s2 = s1; // move
    // s1의 또 다른 복사본을 만들고 s2가 소유자로 할당됨
    let s2 = s1.clone(); // deep copy (힙 문자열의 전체 내용을 복사하기 때문에 느린 작업이다)
    println!("{}", s2); // s2는 s1과 동일한 메모리를 가리킨다
    println!("{}", s1); // s1은 더 이상 사용할 수 없음
    */
    let n1 = 10;
    // 스택 메모리에 또다른 복사본이 생성됨
    let n2 = n1; // this is not move. this called copy
    println!("n1: {}, n2: {}", n1, n2);
}
