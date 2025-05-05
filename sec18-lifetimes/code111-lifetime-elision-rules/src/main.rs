// fn find_biggest_item(strings: &[&str]) -> Option<&str> {
fn find_biggest_item<'a, 'b>(strings: &'a [&'b str]) -> Option<&'b str> {
    let mut longest: Option<&str> = None;
    for item in strings {
        if longest.is_none() || item.len() > longest.unwrap().len() {
            longest = Some(item);
        }
    }

    longest
}
fn main() {
    let strings = ["apple", "banana", "mango"]; //[&str; 3]
    let result = find_biggest_item(&strings); // &[&str; 3] ==> &[&str]

    if let Some(value) = result {
        println!("Biggest item is: {}", value);
    } else {
        println!("Array was empty");
    }
}
