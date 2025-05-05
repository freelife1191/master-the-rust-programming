fn main() {
    /*
    // Define an array of integers with size 5
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Print the array
    println!("Original array: {:?}", numbers);

    // Modify an element of the array
    numbers[2] = 99;

    // Print the modified array
    println!("Modified array: {:?}", numbers);

    // Print the length of the array
    print!("Length of the array: {}", numbers.len());

    // Iterating over the array and print each element
    for num in &numbers {
        println!("{}", num);
    }
    */

    // https://doc.rust-lang.org/std/vec/struct.Vec.html
    // Define an array
    // type of arr : [i32; 4]
    let arr = [1, 2, 3, 4];

    // Convert the array to a vector
    // type of vec1 : Vec<i32>
    let vec1 = arr.to_vec();
    let vec2 = Vec::from(arr);
    let vec3 = Vec::from([1, 2, 3, 4]);
    let vec4 = Vec::from([10, 5]);

    // Print the vector
    println!("{:?}", vec4);
}
