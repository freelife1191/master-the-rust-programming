fn main() {
    /*
    splice()
    */
    // Primary temperature readings (in Celsius) over the last 10 hours
    let mut primary_readings = vec![22.5, 23.0, 22.8, 0.0, 0.0, 0.0, 23.2, 22.9, 22.4, 22.0];

    // Backup readings for hours 4 to 6
    let backup_readings = vec![22.7, 22.6, 23.0];

    // This will not compile because `splice` expects an iterator
    let faulty_readings: Vec<_> = primary_readings.splice(3..6, backup_readings).collect();

    println!("Corrected primary readings: {:?}", primary_readings);
    println!("Faulty readings: {:?}", faulty_readings);

    /*
    append() and extend()
    */
    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    println!("Before append:");
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    println!("Capacity of vec2: {}", vec2.capacity());

    // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append
    vec1.append(&mut vec2);

    println!("\nAfter append:");
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    println!("Capacity of vec2: {}", vec2.capacity());

    /*
    extend()
    */
    let mut vec = vec![1, 2, 3];
    let vec1 = vec![-11, -22];

    vec.extend(4..=6);
    vec.extend([8, 9, 10]);
    vec.extend(&[11, 12]);
    vec.extend(vec1.iter());

    println!("vec: {:?}", vec);
    println!("vec1: {:?}", vec1);
}
