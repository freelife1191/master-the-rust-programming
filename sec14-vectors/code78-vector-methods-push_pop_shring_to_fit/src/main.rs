fn main() {
    /*
    push()
    */

    let mut numbers = Vec::new();

    // numbers.push(1);
    // numbers.push(2);
    // numbers.push(3);
    numbers.push([1, 2, 3]); // [i32, 3]
    numbers.push([2, 0, 0]); // [i32, 3]

    println!("{:?}", numbers);

    /*
    let mut names = Vec::new();
    let name = String::from("Alice");
    names.push(name);

    // The next line would cause a compile error because "name" has been moved
    println!("{}", name);

    println!("{:?}", names); // Outputs: ["Alice"]
    */

    /*
    pop()
    */
    let val = numbers.pop().unwrap();
    println!("{:?}", val); // Outputs: [2, 0, 0]

    if let Some(val) = numbers.pop() {
        println!("{:?}", val); // Outputs: [1, 2, 3]
    } else {
        println!("vec is empty");
    }
    println!("{:?}", numbers); // Outputs: []

    match numbers.pop() {
        Some(val) => println!("{:?}", val),
        None => println!("vec is empty"),
    }

    /*
    shrink_to_fit()
    */
    let mut vec = Vec::with_capacity(10);

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("Before shrink: {}", vec.capacity());
    vec.shrink_to_fit();
    println!("After shrink: {}", vec.capacity());
}
