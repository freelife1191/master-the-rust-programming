/*
 ref and @
 */
#[derive(Debug)]
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
        Person { age: p_age @ 30, name: _ }  => println!("A person with age {} found.", p_age),
        Person { name, age: 35 } => {
            println!("Ram with age 35 found.");
        }
        _ => println!("Not sure who the person is."),
    }

    println!("{:?}", person);
}