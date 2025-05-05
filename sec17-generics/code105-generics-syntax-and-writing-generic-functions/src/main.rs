/*
Generics
언어의 많은 구성이 Generics를 활용할 수 있다
 */

/*
Monomorphization
Summary : function_name<T>


 */
// fn find_max_element(v: &[i32]) -> Option<i32> {
fn find_max_element<T>(v: &[T]) -> Option<T>
    where T: PartialOrd + Copy
{
    if v.is_empty() {
        return None;
    }

    let mut max = v[0]; // max type is i32

    /*
    Pattern matching with references
     */
    for &n in v { // n type is &i32
        if n > max {
            max = n;
        }
    }
    Some(max)
}
fn main() {
    let arr_int = [1, 2, 3, 4, 5];
    let arr_float = [1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Max int: {:?}", find_max_element(&arr_int));
    println!("Max float: {:?}", find_max_element(&arr_float));
}
