#[derive(Debug, Default)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
    height: f32,
    initial: char,
}

fn main() {
    let user = Person::default();
    let pl = Person {
        name: String::from("Kiran"),
        is_male: true,
        ..Default::default()
    };
    println!("{:?}", user);
    println!("{:?}", pl);
}