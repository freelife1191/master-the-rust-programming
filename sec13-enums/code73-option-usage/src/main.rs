fn find_biggest_item<'a>(strings: &'a [&'a str]) -> Option<&'a str> {
    let mut longest: Option<&str> = None;

    for item in strings {
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.is_none
        // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
        if longest.is_none() || (item.len() > longest.unwrap().len()) {
            longest = Some(item);
        }
    }
    longest
}

fn main() {
    /*
    Exercise: 문자열 배열에서 가장 긴 문자열 항목을 반환하는 함수를 작성하세요
    반환 값은 `Option` 유형이어야 합니다
    배열이 비어 있으면 `None`, 그렇지 않으면 `Some(longest_string)`입니다
    */
    let strings = ["Mango", "Banana", "Apple"];
    let biggest_item = find_biggest_item(&strings);

    // let strings = [];
    // let strings = ["Banana"];
    // println!("Biggest item : {}", biggest_item.unwrap());
    /*
    match biggest_item {
        Some(value) => {
            println!("Biggest item : {}", value);
        }
        None => println!("Array is empty"),
    }
    */
    if let Some(value) = biggest_item {
        println!("Biggest item : {}", value);
    } else {
        println!("Array is empty");
    }
}
