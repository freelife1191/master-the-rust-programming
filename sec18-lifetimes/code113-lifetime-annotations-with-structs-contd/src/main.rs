struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}
fn main() {
    /*
    Lifetime annotation with Structs
     */
    // let data1 = "Hello";
    // let data2 = "World";
    let data1 = String::from("Hello");
    let data2 = String::from("World");
    let ret;

    {
        let my_struct;

        my_struct = MyStruct {
            data1: &data1,
            data2: &data2,
        };
        ret = fun(&my_struct);
        println!("ret = {}", ret);
    }
    /*
    `MyStruct`의 인스턴스는 포함된 참조의 수명보다 오래 지속될 수 없다
     */
    // let my_struct = MyStruct {
    //     data1: &data1,
    //     data2: &data2,
    // };

    // let ret = fun(&my_struct);
    // ret = fun(&my_struct);

    // println!("ret = {}", ret);
    println!("{}", ret);
}

/*
제공된 코드의 수명 제약 조건은 `fun` 함수에 의해 반환된 참조의 수명이 최소한 fun에 인수로 전달된 `MyStruct` 참조의 `a` 수명 매개변수만큼 길어야 한다는 것이다
 */
// fn fun<'a, 'b, 'c>(data: &'a MyStruct<'b, 'c>) -> &'a str {
fn fun<'a, 'b, 'c>(data: &'a MyStruct<'b, 'c>) -> &'c str {
    // data.data1
    data.data2
}
