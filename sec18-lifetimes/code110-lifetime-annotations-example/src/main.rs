
// fn find_smallest<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
fn bar<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
// fn bar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
// fn bar(x: &i32, y: &i32) -> &i32 {
    // Function implementation goes here
    x
    // y
    /*
    if x < y {
        x
    } else {
        y
    }
    */
}

fn foo(x: &i32) -> &i32 {
    // Function implementation goes here
    // let y = 50;
    // &y
    // &50 // lifetime is 'static'
    x
}

fn main() {
    let i = 42;
    // let ret = foo(&i);
    let ret;
    {
        let j = 30;
        ret = bar(&i, &j); // 'i = 10, 'j = 5'
    }
    // let j = 30;
    // let ret = find_smallest(&i, &j);

    println!("{}", ret);
}
