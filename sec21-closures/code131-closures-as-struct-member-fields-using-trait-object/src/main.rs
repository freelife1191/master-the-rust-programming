struct MyStruct {
    val: i32,
    // action: Fn(i32) -> i32 // Error
    /*
    특성 이름 앞에 `dyn`을 사용하면 해당 특성을 특성 개체로 사용하겠다는 신호를 보내 해당 특성을 구현하는 모든 유형에 대해 동적 디스패치를 허용합니다
    `Write` 특성을 구현하는 모든 유형을 나타낼 수 있기 때문에 컴파일 타임에는 크기를 알 수 없습니다
     */
    // action: &dyn Fn(i32) -> i32 // Error
    // action: Box<dyn Fn(i32) -> i32>
    action: Box<dyn FnOnce(i32) -> i32>
}
fn main() {
    let y = 10;
    let closure = Box::new(move |x| x + y);

    let s = MyStruct { val: 10, action: closure };
    println!("{}", (s.action)(10)); // 20
}
