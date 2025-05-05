## Passing a struct by reference

```rust
struct Person {
    name: String,
    age: u8,
}

fn update_person_age(person: &mut Person, new_age: u8) {
    // 도트 연산자는 암시적으로 포인터를 역참조하고 구조체 멤버에 액세스합니다
    // 따라서 멤버에 액세스하기 전에 '*' 연산자를 사용하여 포인터를 명시적으로 역참조할 필요가 없습니다
    person.age = new_age;
    // 해당 필드에 액세스하기 위해 구조체에 대한 포인터를 역참조하는 명시적인 방법입니다
    // (*person).age = new_age; // OK
}

fn main() {
    let mut p = Person { name: String::from("Alice"), age: 25 };
    update_person_age(&mut p, 30);
    println!("Name: {}, Age: {}", p.name, p.age);r
}
```
