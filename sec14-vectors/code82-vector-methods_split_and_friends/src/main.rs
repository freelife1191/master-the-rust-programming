fn main() {
    /*
    Methods related to split
    */
    let vec = vec![1, 2, 3, 4, 5];
    let (left, right) = vec.split_at(2);
    println!("Left: {:?}", left); // Left: [1, 2]
    println!("Right: {:?}", right); // Right: [3, 4, 5]

    // let ret = vec.split_at(3);
    // let (l, r) = ret;
    let (l, r) = vec.split_at(3);
    println!("Left: {:?}", l); // Left: [1, 2, 3]
    println!("Right: {:?}", r); // Right: [4, 5]

    /*
    split()
    https://doc.rust-lang.org/core/primitive.slice.html#method.split
    */
    let vec = vec![1, 2, 3, 7, 11, 4, 33, 67, 8, 10];
    let vec_of_ss: Vec<_> = vec.split(|e| e % 2 == 0).collect();
    for ss in vec_of_ss {
        println!("{:?}", ss);
    }

    /*
    splitn()
    https://doc.rust-lang.org/core/primitive.slice.html#method.splitn
    */
    let vec_of_ss: Vec<_> = vec.splitn(2, |e| e % 2 == 0).collect();
    for ss in vec_of_ss {
        println!("{:?}", ss);
    }

    /*
    rsplit()
    https://doc.rust-lang.org/core/primitive.slice.html#method.rsplitn
    */

    /*
    split_off()
    */
    let mut vec = vec![1, 2, 3, 4, 5];

    // Split off the vector form index 3 (0-based), so elements [4, 5] will be transferred
    let split_vec = vec.split_off(3);

    println!("Original vector: {:?}", vec); // prints: [1, 2, 3]
    println!("Split-off vector: {:?}", split_vec); // prints: [4, 5]
}
