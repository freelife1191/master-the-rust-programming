use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    /*
    HashMap<K, V>
    https://doc.rust-lang.org/std/collections/struct.HashMap.html
    */
    // Create a new HashMap
    let mut fruit_prices = HashMap::new();

    // Insert key-value pairs (fruit name as key and price as value)
    fruit_prices.insert("apple", 1.2);
    fruit_prices.insert("banana", 0.8);
    fruit_prices.insert("cherry", 2.5);

    // Access the price of a specific fruit
    let apple_price = fruit_prices.get("apple").unwrap_or(&0.0);

    // Print the price
    println!("The price of an apple is ${}", apple_price);

    // Iterate over all entries and print them
    for (fruit, price) in &fruit_prices {
        println!("{}: ${}", fruit, price);
    }

    /*
    Indexing vs get
    */
    // Accessing values using the indexing syntax
    // This will panic if the key is not present
    println!("Price of an apple: ${}", fruit_prices["apple"]);
    println!("Price of an banana: ${}", fruit_prices["banana"]);

    // Safe way to access values: using the get method
    match fruit_prices.get("orange") {
        Some(price) => println!("Price of an orange: ${}", price),
        None => println!("Orange price doesnt exist"),
    }

    /*
    Iterating over keys and values
    */
    // Iterating and printing all the keys using keys()
    println!("Keys in the map:");
    for key in fruit_prices.keys() {
        println!("{}", key);
    }

    // Iterating and printing all the values using values()
    println!("\nValues in the map:");
    for value in fruit_prices.values() {
        println!("{}", value);
    }

    /*
    Nested HashMaps
    */
    // let mut students_scores: HashMap<String, HashMap<String, i32>> = HashMap::new();

    /*
    Methods of HashMap
    new(), insert(), get(), contains_key(), remove(), len(), is_empty(), clear(), entry(), iter(), keys(), values()
    */

    /*
    entry()
    */
    let mut fruits = HashMap::new();

    // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
    *fruits.entry("apple").or_insert(0) += 1; // 한 줄로 아래 코드들을 대체
    /*
    if fruits.contains_key("apple") {
        // If "apple" exists in the map, get its value and increment it by 1
        let count = fruits.get_mut("apple").unwrap();
        *count += 1;
    } else {
        // If "apple" does not exist in the map, insert it with a value of 1
        fruits.insert("apple", 1);
    }
    */
    println!("{:?}", fruits);

    /*
    or_insert()
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert
    */
    let mut fruits = HashMap::new();
    *fruits.entry("apple").or_insert(0) += 1;
    println!("{:?}", fruits);

    /*
    or_insert_with()
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert_with
    */
    let mut data = HashMap::new();
    data.insert("key", 10);

    // or_insert_with()
    data.entry("key").or_insert_with(|| expensive_computation()); // 키가 존재하지 않는 경우에만 호출됨
    println!("HashMap contents: {:?}", data);
}

fn expensive_computation() -> i32 {
    println!("Expensive computation called!");
    // simulate some complex calculations
    42
}