fn main() {
    // Important methods to be used with Vec
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len
    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity
    let mut v: Vec<i32> = vec![];
    println!("capacity: {}, len: {}", v.capacity(), v.len());
    v.push(1);
    println!("capacity: {}, len: {}", v.capacity(), v.len());

    // Method : Vec::with_capacity();
    // Let's say we expect to store 100 elements
    let expected_elements = 100;
    let mut v = Vec::with_capacity(expected_elements);

    // Add elements to the vector
    for i in 0..expected_elements {
        v.push(i);
    }
}
