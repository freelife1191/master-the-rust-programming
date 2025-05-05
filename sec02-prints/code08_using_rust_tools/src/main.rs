/* Activate this code block and run 'cargo clippy' and
observe the errors/suggestions

// https://doc.rust-lang.org/stable/std/f32/consts/constant.PI.html
// #[deny(clippy::approx_constant)]
fn main() {
    // error: approximate value of `f{32, 64}::consts::PI` found
    // let pi = 3.14;
    // 표준 라이브러리에 있는 PI 상수값을 사용
    let pi = std::f32::consts::PI;
    let area = pi * 4.0 * 4.0;
    println!("{}", area);
}
*/
/* This code block generates warnings related to unused variabels 
   and unused functions and when you run the command 'cargo fix --allow-dirty' 
   some warnings are auto fixed 
*/
fn calc_circle_area(radius: f32) -> f32 {
    3.14 * 4.0 * 4.0
}

fn main() {
    let pi = 3.14;
    let area = pi * 4.0 * 4.0;
    println!("{}", calc_circle_area(10.0));
}


/*
   Running 'cargo build' does not encounter any issues with this code.
   but when you run 'cargo clippy', it suggests using iterators,
   which is the idiomatic way to handle iteration in Rust.

fn main() {
    let array = [1, 2, 3];
    for i in 0..=2 {
        println!("{}", array[i]);
    }
}
*/