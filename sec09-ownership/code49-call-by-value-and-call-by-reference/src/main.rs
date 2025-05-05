// Call by value and Call by reference
fn main() {
    let arr = [10, 20, 30, 40, 50];

    // Call by value
    let max_value = find_greatest_value_by_value(arr);
    println!("Maximum value by value: {}", max_value);

    // Call by reference
    let max_value = find_greatest_value_by_reference(&arr);
    println!("Maximum value by reference: {}", max_value);


    /*
      References to References
     */
    let x = 42;
    let x_ref_1 = &x; //type of 'x_ref_1' : &i32 참조
    let x_ref_2 = &x_ref_1; //type of 'x_ref_2' : &&i32 참조에 참조
    let x_ref_3 = &x_ref_2; //type of 'x_ref_3' : &&&i32 참조에 참조에 참조

    println!("Value of x using 1 level of reference: {}", x_ref_1);
    println!("Value of x using 2 levels of reference: {}", x_ref_2);
    println!("Value of x using 3 levels of reference: {}", x_ref_3);

    println!("Value of x using 1 level of reference: {}", *x_ref_1);
    println!("Value of x using 2 levels of reference: {}", **x_ref_2);
    println!("Value of x using 3 levels of reference: {}", ***x_ref_3);

    /*
      Comparing references
     */
    let x = 42;
    let x_ref = &x; //type of 'x_ref' : &i32
    let y = 42;
    let y_ref = &y; //type of 'y_ref' : &i32

    // 값은 내용에 따라 비교된다
    // 그리고 두 참조(==,>,<,<=,>=,!=,etc)를 비교할 때 Rust는 비교한다
    // 메모리 주소가 아닌 그들이 가리키는 값
    // 따라서 이 경우 조건은 true로 평가된다 포인터나 참조의 비교가 아니다
    if x_ref == y_ref {
        println!("values are the same");
    }

    if *x_ref == *y_ref {
        println!("values are the same");
    }

    let x = 42;
    let x_ref_1 = &x; //type of 'x_ref_1' : &i32
    let x_ref_2 = &x_ref_1; //type of 'x_ref_2' : &&i32

    // This is Error
    // x_ref_2는 x_ref_1의 참조이므로 유형이 일치하지 않는다
    /*
    if x_ref_1 == x_ref_2 {
        println!("values are the same");
    }
    */

    // This is OK
    // x_ref_2를 역참조하면 유형이 x_ref_1와 같아지므로 일치한다
    if x_ref_1 == (*x_ref_2) {
        println!("values are the same");
    }
}

// Call by value
fn find_greatest_value_by_value(arr: [i32; 5]) -> i32 {
    arr[arr.len() -1]
}

// Call by reference
// 변경 가능한 참조인 해당 참조를 사용하면 배열의 내용을 조작하거나 변경할 수 있다 (배열에 직접적인 영향을 미침)
fn find_greatest_value_by_reference(arr: &[i32]) -> i32 {
    arr[arr.len() -1]
}
