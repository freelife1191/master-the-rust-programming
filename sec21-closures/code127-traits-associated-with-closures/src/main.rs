fn main() {
    // let x = 5;
    // let closure = || x * 2;
    let mut x = 5;
    let mut closure = || {
        x *= 2;
        x
    };

    println!("{}", closure());
    println!("{}", closure());

    // call_fn(&closure);
    // call_fn_mut(&closure);
    call_fn_mut(&mut closure);
    // call_fn(&closure); // Error: closure moved
}

fn call_fn<F: Fn() -> i32>(f: F) {
    println!("Fn called: {}", f());
}

fn call_fn_mut<F: FnMut() -> i32>(mut f: F) {
    println!("FnMut called: {}", f());
}