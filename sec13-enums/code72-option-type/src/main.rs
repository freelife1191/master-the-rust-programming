// use std::option::Option;

/*
fn find_value(array: &[i32], target: i32) -> i32 {
    let mut index = 0;
    for value in array {
        if *value == target {
            return index;
        }
        index += 1;
    }
    -1
}
*/
/*
fn find_value(array: &[i32], target: i32) -> i32 {
    for (index, value) in array.iter().enumerate() {
        if *value == target {
            return index as i32;
        }
    }
    -1
}
*/
fn find_value(array: &[i32], target: i32) -> Option<i32> {
    for (index, value) in array.iter().enumerate() {
        if *value == target {
            return Some(index as i32);
        }
    }
    None
}

fn main() {
    /*
    The 'Option' enum type
    https://doc.rust-lang.org/std/option/enum.Option.html
    */
    // let v_1 = Option::Some(20);
    /*
    The Rust Prelude
    https://doc.rust-lang.org/std/prelude/index.html
    */
    // let v_2: Option<String> = Some("Kiran".to_string());
    // let v_3: Option<i32> = None;
    // let v_4: Option::<i32> = None;
    // println!("{:?}", v_1);


    let my_array = [1, 2, 3, 45, 6];
    // println!("index : {}", find_value(&my_array, 2));
    // println!("index : {}", find_value(&my_array, 88));

    // let index = find_value(&my_array, 88);
    // println!("{}", my_array[index as usize]);

    let index = find_value(&my_array, 6);
    if let Some(i) = index {
        println!("Found at index: {}", i);
        println!("{}", my_array[i as usize]);
    } else {
        println!("Value is not found in the array");
    }
}