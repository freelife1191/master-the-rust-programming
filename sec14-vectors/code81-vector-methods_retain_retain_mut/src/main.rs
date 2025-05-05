fn retain_positive(x: &i32) -> bool {
    *x > 0
}

fn retain_and_modify_positives(x: &mut i32) -> bool {
    if *x > 0 {
        *x += 10;
        true
    } else {
        false
    }
}

fn main() {
    /*
    retain() and retain_mut()
    */
    let mut numbers = vec![-3, -2, -1, 0, 1, 2, 3];

    // 그러면 Vec에 양수만 유지됩니다
    // numbers.retain(|x| *x > 0);
    // numbers.retain(retain_positive);
    // numbers.retain_mut(retain_and_modify_positives); // Output: [11, 12, 13]
    numbers.retain_mut(|x| if *x > 0 {
        *x += 10;
        true
    } else {
        false
    });
    println!("{:?}", numbers); // prints: [1, 2, 3]

    // 길이가 6보다 작은 요소를 유지합니다
    let mut planets = vec!["Mercury", "Jupiter", "Earth", "Saturn", "Mars"];
    planets.retain(|planet| planet.len() < 6);
    println!("{:?}", planets); // prints: ["Earth", "Mars"]

    // 길이가 6보다 큰 요소를 유지합니다
    // 길이 정보를 추가하여 유지된 요소를 수정합니다
    // e.g: Hoth: 4
    let mut star_wars_planets = vec![
        String::from("Tatooine"),
        String::from("Coruscant"),
        String::from("Hoth"),
        String::from("Dagobah"),
        String::from("Mustafar"),
    ];

    star_wars_planets.retain_mut(|planet| {
        if planet.len() > 6 {
            *planet += &format!(": {}", planet.len());
            true
        } else {
            false
        }
    });
    println!("{:?}", star_wars_planets); // prints: ["Tatooine: 8", "Coruscant: 9", "Dagobah: 7", "Mustafar: 8"]
}

/*
Internally, 'retain' iterates over each element of the vector.
For each element, the closure will be applied.
For -3, the closure returns false (not retained).
For -2, the closure returns false (not retained).
For -1, the closure returns false (not retained).
For 0, the closure returns false (not retained).
For 1, the closure returns true.
For 2, the closure returns true.
For 3, the closure returns true.
*/