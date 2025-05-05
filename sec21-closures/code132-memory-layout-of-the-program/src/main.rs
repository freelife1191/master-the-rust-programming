struct MyStruct {
    val: i32,
    // https://doc.rust-lang.org/stable/std/ops/trait.Fn.html
    action: Box<dyn Fn(i32) -> i32>
}
fn main() {
    let y = 10;
    let closure = Box::new(move |x| x + y); // Fn

    let s = MyStruct { val: 10, action: closure };
    println!("{}", (s.action)(10)); // 20
}
