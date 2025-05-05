# HashMap

```rust
use std::collections::HashMap;

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
}
```

## Indexing vs get

```rust
use std::collections::HashMap;

fn main() {
    // Create a HashMap to store fruit prices
    let mut fruit_prices = HashMap::new();

    // Insert some fruits with their prices
    fruit_prices.insert("apple", 1.20);
    fruit_prices.insert("banana", 0.80);
    fruit_prices.insert("cherry", 2.50);

    // Accessing values using the indexing syntax
    // This will panic if the key is not present
    println!("Price of an apple: ${}", fruit_prices["apple"]);
    println!("Price of an banana: ${}", fruit_prices["banana"]);

    // Safe way to access values: using the get method
    match fruit_prices.get("orange") {
        Some(price) => println!("Price of an orange: ${}", price),
        None => println!("Orange price doesnt exist"),
    }
}
```

## Iterating over HashMap

HashMap의 키-값 쌍에 대한 참조를 반복합니다. 이렇게 하면 소유권 이전이 방지됩니다  
각 반복 동안 for 루프는 HashMap에서 다음 (키, 값) 튜플을 검색하고 튜플을 (과일, 가격) 패턴과 일치시켜 구조를 분해합니다

```rust
for (fruit, price) in &fruit_prices {
    println!("{}: ${}", fruit, price);
}
```

## Nested HashMaps

HashMap 의 '값'은 다른 HashMap 일 수 있습니다

`let mut students_scores: HashMap<String, HashMap<String, i32>> = HashMap::new();`

Rust에서 중첩된 HashMap 을 구현하여 다양한 과목에 대한 학생 점수를 표와 같은 형식으로 저장하고 표시합니다

## Methods of HashMap

- **new()**: 비어 있는 새 HashMap을 만듭니다
- **insert()**: 키-값 쌍을 맵에 삽입합니다
- **get()**: 키에 해당하는 값에 대한 참조를 반환합니다
- **remove()**: 지도에서 키를 제거합니다
- **contains_key()**: 맵에 키가 있는지 확인합니다
- **is_empty()**: 지도에 요소가 없으면 true 를 반환합니다
- **len()**: 맵의 요소 수를 반환합니다
- **clear()**: 맵을 지우고 모든 키-값 쌍을 제거합니다
- **entry()**: 내부 조작을 위해 지도에서 지정된 키의 해당 항목을 가져옵니다
- **keys()**: 지도의 키에 대한 반복자를 반환합니다
- **values()**: 지도 값에 대한 반복자를 반환합니다

### entry()

- 기존 키의 값을 수정하거나 키가 없는 경우 새 키-값 쌍을 삽입하려는 시나리오에 유용합니다 (내부 조작)

- "apple"이 HashMap에 없으면 값이 1인 "apple" 키를 추가합니다
- "apple"이 이미 HashMap에 있는 경우 값을 1씩 증가시킵니다

```rust
fn main() {
    let mut fruits = HashMap::new();

    // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
    // *fruits.entry("apple").or_insert(0) += 1; // 한 줄로 아래 코드들을 대체
    if fruits.contains_key("apple") {
        // If "apple" exists in the map, get its value and increment it by 1
        let count = fruits.get_mut("apple").unwrap();
        *count += 1;
    } else {
        // If "apple" does not exist in the map, insert it with a value of 1
        fruits.insert("apple", 1);
    }
    
    println!("{:?}", fruits);
}
```

### or_insert()

- 항목이 Vacant인 경우 이 메소드는 키에 대해 지정된 값을 삽입하고 해당 값에 대한 변경 가능한 참조를 반환합니다
- 항목이 Occupied인 경우 기존 값에 대한 변경 가능한 참조를 반환합니다

https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert

```rust
fn main() {
    let mut fruits = HashMap::new();
    *fruits.entry("apple").or_insert(0) += 1;
    println!("{:?}", fruits);
}
```

### or_insert_with()

- `or_insert()`와 비슷하지만 `or_insert_with()` 메서드는 키가 HashMap 에 존재하지 않는 경우 삽입할 값을 반환하는 클로저를 기대합니다

https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert_with

```rust
use std::collections::HashMap;

fn expensive_computation() -> i32 {
    println!("Expensive computation called!");
    // simulate some complex calculations
    42
}

fn main() {
    let mut data = HashMap::new();
    data.insert("key", 10);

    // or_insert_with()
    data.entry("key").or_insert_with(|| expensive_computation()); // 키가 존재하지 않는 경우에만 호출됨
    println!("HashMap contents: {:?}", data);
}
```