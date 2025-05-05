/*
`Fn` 특성은 환경을 변경하지 않는 클로저를 위한 것이기 때문에 `Fn` 클로저를 기대하는 함수에 `FnMut`을 구현하는 클로저를 전달할 수 없습니다
 */
// fn apply<F>(f: F, arg: i32) -> i32
// fn apply<F>(mut f: F, arg: i32) -> i32
fn apply<F>(f: &mut F, arg: i32) -> i32
    // where F: Fn(i32) -> i32
    where F: FnMut(i32) -> i32
    // where F: FnOnce(i32) -> i32
{
    // f(arg);
    f(arg)
}

fn main() {
    /*
    let mut y = 2;
    let mut multiply = |x| { y += x; y}; // Error

    let result = apply(&mut multiply, 4);
    let result = apply(&mut multiply, 4);
    println!("{}", result);
    */
    // let s = String::from("22");
    let mut s = String::from("22");

    // 이 클로저는 `move` 키워드를 사용하여 환경을 소비합니다
    let closure = move || {
        // println!("{}", s);
        s += "11";
        println!("{}", s);
        s
    };

    execute_closure(closure);
    // println!("{}", s); // Error
}

// fn execute_closure<F: FnOnce()>(f: F) {
// fn execute_closure<F: Fn()>(f: F) {
// fn execute_closure<F: FnMut() -> String>(f: F) {
fn execute_closure<F: FnOnce() -> String>(f: F) {
    f();
    // f(); // Error
    // f();
    // f();
}