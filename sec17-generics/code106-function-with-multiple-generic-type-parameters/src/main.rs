/*
Generic function with two type parameters of same type T
 */
/*
fn combine<T>(a: T, b: T) -> Vec<T> {
    let mut v = Vec::new();
    v.push(a);
    v.push(b);
    v
}
*/

/*
Generic function with two different type parameters T and U
 */
fn combine<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}
fn main() {
    /*
    let v1 = combine(3, 4);
    println!("{:?}", v1); // [3, 4]
    let v2 = combine("Hello".to_string(), "World".to_string());
    println!("{:?}", v2); // ["Hello", "World"]
    */

    let t1 = combine(3, "three");
    let t2 = combine("two", 2.0);
    let t3 = combine("one", "two".to_string());
    println!("{:?}\n{:?}\n{:?}", t1, t2, t3); // (3, "three") ("two", 2.0) ("one", "two")
}
