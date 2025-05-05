fn main() {
    /*
    Slice of a vector
    */
    let vec = vec![1, 2, 3, 4, 5];
    let slice = &vec[1..4]; // A slice containing[2, 3, 4]

    let vec = vec![0, 1, 2, 3, 4];

    // a slice containing all the elements: [0, 1, 2, 3, 4]
    let a = &vec[..];

    // a slice starting from the second element: [0, 1, 2, 3, 4]
    let b = &vec[1..];

    // a slice of the first three elements: [0, 1, 2]
    let c = &vec[..3];

    // a slice starting from the second and up to the third element: [1, 2]
    let d = &vec[1..3];

    // https://doc.rust-lang.org/core/primitive.slice.html#method.get_mut
}
