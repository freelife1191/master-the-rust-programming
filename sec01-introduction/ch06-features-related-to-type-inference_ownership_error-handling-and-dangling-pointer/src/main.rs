fn main() {
    let s = String::from("Hello");
    let t = s;
    println!("t is {}", t);
    // println!("s is {}", s); // 오류: 's'는 이제 더 이상 유효하지 않습니다.

    fun1();
}

fn fun1() {
    let mut s = String::from("Hello");
    let t = String::from("World");
    /*
        's'에 't'를 할당하면 's'의 데이터가 't'로 이동됩니다.
        's'는 이제 더 이상 유효하지 않습니다.
     */
    s = t;
    // println!("t is {}", t); // 오류: 't'는 이제 더 이상 유효하지 않습니다.
    println!("s is {}", s);

    /* 오류: 다시 초기화하지 않으면 't'를 사용할 수 없습니다 */
    // println!("t is {}", t);
}