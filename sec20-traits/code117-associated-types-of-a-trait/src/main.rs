trait Animal {
    /*
    특성 메서드 서명의 'self'는 필수이다.
    특성을 구현하는 유형의 인스턴스를 나타내는 데 사용된다.
    개체의 속성에 액세스하여 수정하거나 개체에 대한 다른 메서드를 호출할 수 있다.
     */
    fn make_sound(&self);
    type Weight;
    /*
    `Self`는 특성을 구현하는 열거형의 구조체 유형을 참조하는 Rust 키워드이다
    예를 들어 Animal 특성을 구현하는 Dog 구조체가 있는 경우 특성 메서드의 `Self`는 Dog 유형을 참조한다
     */
    fn set_weight(&mut self, weight: Self::Weight);
    fn get_weight(&self) -> Self::Weight;
    fn set_age(&mut self, age: u8);
    fn get_age(&self) -> u8;
}

#[derive(Default)]
struct Dog {
    age: u8,
    weight: f32,
}

impl Animal for Dog {
    fn make_sound(&self) {
        // Dog 구조체는 Animal 특성을 구현한다
        println!("멍멍");
    }
    // 특성의 구현은 연관된 유형에 대한 구체적인 유형을 제공한다
    type Weight = f32;
    fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }
    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
    fn get_age(&self) -> u8 {
        self.age
    }
}

#[derive(Default)]
struct Cat {
    age: u8,
    weight: f32,
}

impl Animal for Cat {
    fn make_sound(&self) {
        // Cat 구조체는 Animal 특성을 구현한다
        println!("야옹");
    }
    // 특성의 구현은 연관된 유형에 대한 구체적인 유형을 제공한다
    type Weight = f32;
    fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }
    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, age: u8) {
        self.age = age;
    }
    fn get_age(&self) -> u8 {
        self.age
    }
}

/*
producer_sound() 함수는 dyn Animal 유형의 특성 개체에 대한 참조를 취하며, 이는 Animal 특성을 구현하는 모든 개체에 대한 참조일 수 있다
 */
fn produce_sound<T>(animal: &dyn Animal<Weight = T>) {
    // 특성을 구현하는 유형의 인스턴스를 나타내는 데 사용된다
    animal.make_sound();
}

fn main() {
    // produce_sound(&Dog);
    // produce_sound(&Cat);

    let mut my_dog = Dog::default();
    produce_sound(&my_dog);
    my_dog.set_age(10);
    println!("age: {}", my_dog.get_age());
}
