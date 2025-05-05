/*
Exercise: 가격대를 기준으로 'Car' 컬렉션 반복(Iterating over Car collection based on price range)

- `CarCollection`이라는 사용자 정의 데이터 유형을 갖춘 프로그램을 작성합니다.
  이 컬렉션은 제조사, 모델, 가격과 같은 속성을 각각 포함하는 `Car` 객체 목록을 저장합니다.
- 핵심 요구 사항은 `CarCollection`에 대한 `IntoIterator` 특성을 세 가지 형식(값 기준, 불변 참조 기준, 가변 참조 기준)으로 구현하는 것입니다.
- 그러나 독특한 변형이 있습니다.
  반복자의 `next` 메서드는 지정된 가격 범위 내의 자동차에 대해 반복되어야 합니다.
  이는 `CarCollectio`n이 반복될 때 가격이 특정 범위 내에 있는 자동차(예: 20,000~50,000 USD 사이의 가격이 책정된 자동차)만 산출해야 함을 의미합니다.
*/

#[derive(Debug)]
struct Car {
    make: String,
    model: String,
    price: u32,
}

#[derive(Debug)]
struct CarCollection {
    cars: Vec<Car>,
    price_range: (u32, u32), // Price range (min, max)
}

impl CarCollection {
    fn new(cars: Vec<Car> , price_range: (u32, u32)) -> Self {
        CarCollection { cars, price_range }
    }
}

// Custom iterator for by-value iteration
struct CarPriceRangeIteratorByValue {
    // https://doc.rust-lang.org/stable/std/vec/struct.IntoIter.html
    remaining_cars:  std::vec::IntoIter<Car>,
    price_range: (u32, u32),
}


impl Iterator for CarPriceRangeIteratorByValue {
    type Item = Car;
    fn next(&mut self) -> Option<Self::Item> {
        //type of car is &Car
        self.remaining_cars.find(|car| (*car).price >= (*self).price_range.0 && car.price <= self.price_range.1)

        /*
                while let Some(car) = self.remaining_cars.next() {
                    if car.price >= self.price_range.0 && car.price <= self.price_range.1 {
                        return Some(car);
                    }
                }

                None
        */
    }
}

// Implement `IntoIterator` for `CarCollection` (by value)
impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter = CarPriceRangeIteratorByValue;
    /* `Iterator` 특성을 구현하는 유형의 인스턴스를 반환해야 합니다.
       `Iterator` 특성과 관련된 `Item` 유형은 반복자가 생성할 요소의 유형을 지정합니다. */
    fn into_iter(self) -> Self::IntoIter {
        CarPriceRangeIteratorByValue {
            remaining_cars: self.cars.into_iter(),
            price_range: self.price_range,
        }
    }
}



#[cfg(feature = "Not_ready")]
// Implement `IntoIterator` for `&CarCollection` (by immutable borrow)
impl IntoIterator for &CarCollection {
    type Item = &Car;
    type IntoIter = Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {

    }
}


#[cfg(feature = "Not_ready")]
// Implement `IntoIterator` for `&mut CarCollection` (by mutable borrow)
impl IntoIterator for &mut CarCollection {
    type Item = &mut Car;
    type IntoIter = Iterator<Item = Self::Item>;

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



    let car_collection_1 = CarCollection::new(cars, (8000, 13000));

    println!("iterate over car_collection by value");
    for car in car_collection_1 {
        println!("Found car: {} {}, Price: {}", car.make, car.model, car.price);
    }


    /*
       let mut car_collection_2 = CarCollection::new(cars, (8000, 15000));

       println!("iterate over car_collection by immutable borrow");
       for car in &mut car_collection_2 {
            println!("Found car: {} {}, Price: {}", car.make, car.model, car.price);
            car.price = 100;
       }
   */



}
