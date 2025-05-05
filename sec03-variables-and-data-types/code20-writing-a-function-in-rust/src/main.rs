fn main() {
    // greet("Ram");
    let greeting = greet("Ram");
    println!("{}", greeting);

    // module_1::convert_kg_to_grams(4 as f64);
    println!("{}", convert_kg_to_grams(4 as f64));
    println!("{}", concatenate_strings("Good", " Morning"));
    // println!("{}", find_biggest_string("Good", "Morning")); // Error
}

fn greet(name: &str) -> String {
    // println!("Hello, {}!", name);
    format!("Hello, {}!", name)
}
/*
pub fn convert_kg_to_grams(in_kg: f64) -> f64 {
    let grams = in_kg * 1000.0;
    return grams;
}
*/
fn convert_kg_to_grams(in_kg: f64) -> f64 {
    in_kg * (1000 as f64)
}

fn concatenate_strings(first: &str, second: &str) -> String {
    first.to_string() + second
}

/*
fn find_biggest_string(first: &str, second: &str) -> &str {
    if first.len() > second.len() {
        first
    } else {
        second
    }
}
*/

/*
fn convert_kg_to_grams(in_kg: i32) {
    println!("{}", in_kg);
}
*/
/*
mod module_1 {
    pub fn convert_kg_to_grams(in_kg: f64) {
        println!("{}", in_kg);
    }
}

mod module_2 {
    pub fn convert_kg_to_grams(in_kg: i64) {
        println!("{}", in_kg);
    }
}
*/