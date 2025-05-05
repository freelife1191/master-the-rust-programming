fn main() {
    // let y = 1;

    // let c1 = |x| x + y;
    // let c2 = |x| x + y;
    // let vec_of_closures = vec![c1, c2];

    // let c1: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + y);
    // let c2: Box<dyn Fn(i32) -> i32> = Box::new(|x| x + y);
    // let vec_of_closures: Vec<Box<dyn Fn(i32) -> i32>> = vec![c1, c2];

    // let c1 = |x| x + 1;
    // let c2 = |x| x + 1;
    let c1: fn(i32) -> i32 = |x| x + 1;
    let c2: fn(i32) -> i32 = |x| x + 1;
    let vec_of_closures = vec![c1, c2];

    println!("{}", vec_of_closures[0](1)); // 2
}
