/*
필드 data1은 `&'a str` 유형으로, 수명이 'a인 문자열 조각에 대한 참조를 나타낸다
마찬가지로 필드 data2는 `&'b str` 유형으로, 수명이 'b인 문자열 조각에 대한 참조를 나타낸다

일반 수명 매개변수를 사용하면 `MyStruct` 구조체는 해당 필드에 대해 수명이 다른 참조를 허용할 수 있다
이를 통해 data1과 data2의 수명이 서로 다르거나 독립적일 수 있는 구조체의 인스턴스를 만들 수 있다
 */
struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}
fn main() {
    /*
    Lifetime annotation with Structs
     */
    let data1 = "Hello";
    let data2 = "World";

    /*
    `MyStruct`의 인스턴스는 포함된 참조의 수명보다 오래 지속될 수 없다
     */
    let my_struct = MyStruct {
        data1: &data1,
        data2: &data2,
    };

    println!("Data 1: {}", my_struct.data1);
    println!("Data 2: {}", my_struct.data2);
}
