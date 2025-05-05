fn print_value(arg: &i32) {
    println!("{}", *arg);
}

fn main() {
    let mut num1 = 50; // mutable referent
    let ref1 = &num1; // immutable borrow (type of 'ref1' is '&i32')
    // 변수를 변경 가능으로 빌려오고 이후 이를 불변으로 빌려오려고 하면 컴파일 에러가 발생
    let ref2 = &mut num1; // mutable borrow (type of 'ref2' is '&mut i32')
    // *ref2 = 100;
    // println!("num1: {}", num1);
    // println!("ref_of_num1: {}", *ref2);
    // print_value(&num1); // immutable borrow

    // *ref_of_num1 += 1; // 변경 가능으로 빌린 경우 다시 사용 불가
    *ref2 = 200; // 변경 가능으로 빌린 경우 다시 사용 불가
    println!("{}", num1);
    println!("{}", ref1); // num1은 ref1에서 불변으로 빌렸는데 ref2에서 다시 변경 가능으로 빌릴수 없다
}
