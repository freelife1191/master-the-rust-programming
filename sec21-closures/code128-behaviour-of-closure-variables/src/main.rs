fn main() {
    /*
    let x = 10;
    let closure = || println!("{}", x);

    let c1 = closure;
    let c2 = closure;

    c1();
    c2();
    closure();
    */

    /*
    let mut x = 10;

    let mut closure = || {
        x += 1;
        println!("{}", x);
    };

    let mut c1 = closure; // move
    */
    // let mut x = 10;
    // let y = &mut x;
    // let z = y;

    // let x = "10".to_string();
    let x = "10";

    // let closure = || println!("{}", x);
    let closure = move || println!("{}", x);

    let c1 = closure;
    c1();
    closure();
}
