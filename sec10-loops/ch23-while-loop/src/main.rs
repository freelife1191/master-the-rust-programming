fn main() {
    /*
      원본 배열 값 변경 예시
     */
    let mut numbers = [ 23, 34, 5, 6, 7, 88 ];
    let mut i = 0;
    // while 문은 반복자를 사용하지 않음
    // for 문은 범위 기반 반복 수행 while 문은 조건 기반 반복 수행시 각각 사용
    while i < numbers.len() {
        if numbers[i] < 10 {
            numbers[i] = 0;
        }
        i += 1;
    }

    println!("{:?}", numbers);

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;

    while index < numbers.len() {
        let number = numbers[index];

        if number % 2 == 0 {
            println!("{:2} is even", number);
        } else {
            println!("{:2} is odd", number);
        }

        index += 1;
    }
}
