
// Destructuring a struct
/*
struct Point {
    x: i32,
    y: i32,
}

// write a pattern to process x when x = 20
fn main() {
    let p = Point { x: 20, y: 10 };

    match p {
        // Point { x: x1, ..} if x1 == 20 => {
        // Point { x @ 20, ..} => {
        Point { x: 20, ..} => {
            // println!("x is {}", x);
            println!("x is 20");
        }
        _ => (),
    }
}
*/

// Matching multiple fields with guard
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person {
        name: "Ram".to_string(),
        age: 35,
    };

    match person {
        Person { name: _, age: 30 }  => println!("A person with age 30 found."),
        Person { name, age: 35 } if name == "Ram" => {
            println!("Ram with age 35 found.");
        }
        _ => println!("Not sure who the person is."),
    }
}