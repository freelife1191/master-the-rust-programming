fn main() {
    let my_array: [f64; 3] = [2.5, 4.0, 3.8];
    let my_array: [char; 2] = ['+', '-'];

    // What's the type of this array?
    let my_array = [1, 2, 3_u8, 67, 89];

    // Printing array
    let my_array = [1, 2, 3];
    // `[{integer}; 3]` doesn't implement `std::fmt::Display`
    // println!("{}", my_array); // Error
    println!("{:?}", my_array);
    println!("{:#?}", my_array);

    /*
     creating an array and initialize all elements to zero
     [V, N] V=value, N=repeat expression
     V is repeated N times
     */
    let array2: [i32; 10] = [0; 10];
    /*
     Creates an array of type u8 of 4 elements and initialize all elements to 5
     */
    let array3 = [5_u8; 4];
    // let buffer = [0_u8; 1024];
    // println!("{:?}", buffer);
    let buffer = [4.5; 10];
    println!("{:?}", buffer);

    // Array indexing
    let array1 = [5, 4, 3, 2, 1];
    let element = array1[3];
    println!("{}", element);

    // array1[3] = 10; // Error

    let mut array2 = [5_u8;5];
    array2[4] = 10;
    println!("{:?}", array2);

    let my_array = [10, -67, 88, -5, -2, 99, 132, 42];
    let mut sum = 0;
    for e in my_array {
        sum += e;
    }
    println!("{}", sum);
}
