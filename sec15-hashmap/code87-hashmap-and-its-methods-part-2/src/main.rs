use std::collections::HashMap;

// https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace
fn count_word(text: &str) -> HashMap<String, u32> {
    let mut word_count = HashMap::new();
    // 반복자의 필터 메소드는 제공된 클로저가 true 를 반환하는 요소만 생성합니다
    let processed_text: String = text.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).collect();
    /*
    for word in text.split_whitespace() {
        word_count.entry(word).and_modify(|c| *c += 1).or_insert(1);
    }
    */
    for word in processed_text.split_whitespace() {
        word_count.entry(word.to_string()).and_modify(|c| *c += 1).or_insert(1);
    }
    word_count
}
fn main() {
    /*
    and_modify()
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.and_modify
    */
    let mut fruits = HashMap::new();
    fruits.insert("apple", 2);
    fruits.entry("apple").and_modify(|value| *value += 1).or_insert(1);
    println!("{:?}", fruits);

    let text = "this is a sample text, this text is just a sample";
    let word_count = count_word(text);
    for (w, c) in &word_count {
        println!("{} : {}", w, c);
    }
}
