/*
    Manipulating array with slice methods
*/
fn main() {
    // https://doc.rust-lang.org/std/primitive.slice.html#method.reverse
    // 정수로 배열을 초기화하고 요소 순서를 반대로 바꿉니다.
    let mut my_array = [1, 2, 555, 4, 5];
    my_array.reverse();
    println!("{:?}", my_array);

    // https://doc.rust-lang.org/std/primitive.slice.html#method.sort
    // 배열을 정렬하여 가장 큰 요소를 찾습니다.
    my_array.sort();
    // https://doc.rust-lang.org/stable/std/primitive.slice.html#method.len
    let len = my_array.len();
    println!("Biggest number :{}", my_array[len - 1]);

    // 두 개의 배열을 초기화하고 이를 세 번째 배열로 연결합니다.
    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    // https://doc.rust-lang.org/stable/std/primitive.slice.html#method.concat
    let array3 = [array1, array2].concat();
    println!("{:?} + {:?} = {:?}", array1, array2, array3);

    // 단일 배열을 지정된 인덱스에서 두 개로 분할합니다.
    let my_array2 = [34, 56, 7, 89, 5, 4, 3, 6, 73, 2];
    // https://doc.rust-lang.org/stable/std/primitive.slice.html#method.split_at
    let (a, b) = my_array2.split_at(3);  // 인덱스 3에서 배열 my_array2를 분할합니다.
    println!("a = {:?}", a);  // 분할된 배열의 첫 번째 부분을 표시합니다.
    println!("b = {:?}", b);  // 분할된 배열의 두 번째 부분을 표시합니다.
}
