trait Animal {
    /*
    특성 메서드 서명의 'self'는 필수이다.
    특성을 구현하는 유형의 인스턴스를 나타내는 데 사용된다.
    개체의 속성에 액세스하여 수정하거나 개체에 대한 다른 메서드를 호출할 수 있다.
     */
    fn make_sound(&self);
    fn set_age(&mut self, age: u8) {
        println!("This feature is not supported")
    }
    fn get_age(&self) -> u8 {
        println!("This feature is not supported");
        0
    }
}

#[derive(Default)]
struct Dog {
    age: u8
}

impl Animal for Dog {
    fn make_sound(&self) {
        // Dog 구조체는 Animal 특성을 구현한다
        println!("멍멍");
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
    age: u8
}

impl Animal for Cat {
    fn make_sound(&self) {
        // Cat 구조체는 Animal 특성을 구현한다
        println!("야옹");
    }
}

/*
producer_sound() 함수는 dyn Animal 유형의 특성 개체에 대한 참조를 취하며, 이는 Animal 특성을 구현하는 모든 개체에 대한 참조일 수 있다
 */
fn produce_sound(animal: &dyn Animal) {
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
