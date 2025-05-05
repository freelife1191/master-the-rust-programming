# Struct encapsulation

```rust
mod car_module {
    // #[derive(Debug)] // 필요시 Debug Trait 추가
    pub struct Car {
        make: String,  // 기본적으로 private field
        model: String, // 기본적으로 private field
        year: u32,     // 기본적으로 private field
    }

    impl Car {
        // Public constructor method
        pub fn new(make: String, model: String, year: u32) -> Self {
            // 여기서 기본적인 유효성 검사를 추가할 수 있음
            Car { make, model, year }
        }

        // Public getter for 'make'
        // 소유권이 필요하지 않다면 효율성을 위해 보통 &str 반환
        pub fn get_make(&self) -> &String {
            &self.make
        }

        // Public getter for 'model'
        // 보통 &str 반환
        pub fn get_model(&self) -> &String {
            &self.model
        }

        // Public method to update 'year'
        // 연도가 적절한 범위 내에 있는지 확인하는 유효성 검사 포함
        pub fn update_year(&mut self, new_year: u32) {
            // 현재 연도 검증 등을 위해 time 관련 라이브러리 사용 고려
            if new_year > 1900 && new_year <= 2023 { // 상한값이 하드코딩됨
                self.year = new_year;
            }
            // 선택적으로 성공/실패 여부를 나타내는 Result나 bool 반환 가능
        }
    }
}


fn main() {
    let mut my_car = car_module::Car::new("Toyota".to_string(), "Camry".to_string(), 2020);
    println!("Car Make: {}", my_car.get_make());
    println!("Car Model: {}", my_car.get_model());
}
```

```bash
Car Make: Toyota
Car Model: Camry
```