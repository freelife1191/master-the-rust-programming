fn main() {
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
