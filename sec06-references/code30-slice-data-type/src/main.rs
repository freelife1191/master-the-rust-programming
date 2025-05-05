fn main() {
    // slice of an array
    let my_array: [i32; 4] = [1, 2, 3, 4];
    // let s1 = &my_array[1..3]; // borrow 를 사용해야됨 item 2, 3
    // borrow 를 사용해야됨 item 2, 3 and 4
    let s1 = &my_array[1..=3]; // s1 is a slice whose type is &[i32]
    let s2 = &my_array[..]; // 1, 2, 3, 4
    let s3 = &my_array[0..=1]; // 1, 2

    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);

    let mut my_array = [-56, 22, 3, 444, 90];
    let s1 = &mut my_array[1..=3];
    // println!("{}", s1[5]); // panic! out of bounds
    s1[2] = 100;
    println!("{:?}", my_array); // [-56, 22, 3, 100, 90]

    // Iterating over the slice of an array
    let my_array = [10, -67, 88, -5, -2, 99, 132, 42];
    let my_slice = &my_array[0..=3]; // 10, -67, 88, -5
    println!("{:?}", my_slice);
    let mut sum = 0; // 슬라이스의 모든 요소의 합
    // type of my_slice is &[i32]
    // i is a loop variable of type &i32
    for i in my_slice {
        sum += *i;
    }
    println!("sum: {}", sum); // 26
}
