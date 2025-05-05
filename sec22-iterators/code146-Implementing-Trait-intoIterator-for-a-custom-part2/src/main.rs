struct Car {
    make: String,
    model: String,
    price: u32,
}

struct CarCollection {
    cars: Vec<Car>,
    price_range: (u32, u32), // Price range (min, max)
}

impl CarCollection {
    fn new(cars: Vec<Car>, price_range: (u32, u32)) -> Self {
        CarCollection { cars, price_range }
    }
}

// https://doc.rust-lang.org/stable/std/iter/trait.IntoIterator.html
// Implement `IntoIterator` for `CarCollection` (by value)
impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {

    }
}

// Implement `IntoIterator` for `&CarCollection` (by immutable borrow)
impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {

    }
}

// Implement `IntoIterator` for `&mut CarCollection` (by mutable borrow)
impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {

    }
}

fn main() {
    let cars = vec![
        Car { make: "Maruti Suzuki".to_string(), model: "Swift".to_string(), price: 8000 },
        Car { make: "Honda".to_string(), model: "City".to_string(), price: 12000 },
        Car { make: "Tata Motors".to_string(), model: "Nexon".to_string(), price: 10000 },
        // Add more cars if needed
    ];

    let car_collection_1 = CarCollection::new(cars, (8000, 10000));

    println!("iterate over car_collection by value");
    for car in car_collection_1 {
        println!("Found car: {} {}, Price: {}", car.make, car.model, car.price);
    }

    println!("iterate over car_collection by immutable or mutable borrow");
    let car_collection_2 = CarCollection::new(cars, (8000, 10000));

    for car in &car_collection_2 {
        println!("Found car: {} {}, Price: {}", car.make, car.model, car.price);
    }
}
