fn main() {
    /*
    let mut v = Vec::with_capacity(4); // Start with a capacity of 4
    for i in 1..=4 {
        v.push(format!("String {}", i));
    }

    // At this point, the vector's length equals its capacity
    let vector_base_addr_in_heap_1 = v.as_ptr();
    println!("Heap address (capacity {}): {:p}", v.capacity(), vector_base_addr_in_heap_1);

    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_ptr
    v.push("Another String".to_string());
    let vector_base_addr_in_heap_2 = v.as_ptr();
    println!("Heap address (capacity {}): {:p}", v.capacity(), vector_base_addr_in_heap_2);
    */

    /*
    Is Vector copy or move?
    */
    /*
    let v = vec![1, 2, 3, 4];
    let v1 = v;
    println!("{:?}", v);
    */

    /*
    Vector indexing
    */
    /*
    let v = vec![1, 2, 3];
    println!("{}", v[0]);
    // This will panic at runtime with an 'index out of bounds' error
    println!("{}", v[5]);
    */
    /*
    let v = vec!["Sun".to_string(), "Mon".to_string(), "Tue".to_string()];
    let s = &v[0];
    println!("{}", s);
    */

    /*
    Safe to vector indexing
    */
    let mut vec = vec![1, 2, 3];

    // Using get
    // val_ref is of type Option<&i32>
    let val_ref = vec.get(1);
    if let Some(val) = val_ref {
        println!("Value: {}", val);
    }

    // Using get_mut
    // val_mut_ref is of type Option<&mut i32>
    let val_mut_ref = vec.get_mut(2);
    if let Some(val) = val_mut_ref {
        // Dereferencing and modifying the value in place
        *val *= 10;
    }
    println!("{:?}", vec);
}
