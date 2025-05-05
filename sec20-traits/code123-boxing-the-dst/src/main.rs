trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

// struct Circle 이 제공하는 `trait` 구현
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

// struct Rectangle 에 의해 제공되는 `trait` 구현
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// 이 함수는 'trait objects' 를 사용합니다.

fn print_area(shape: Box<dyn Shape>) {
    println!("Area: {}", shape.area());
}


fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };

    // 여기서 `vector`는 `trait objects`를 소유합니다.
    // Box는 힙에 할당된 스마트 포인터 유형이다
    // 동적으로 크기를 저장하는 데 사용할 수 있는 trait objects(dyn Trait)와 같은 유형(DST)및 슬라이스([T]).
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle)];

    // type of 'shape' is &dyn Shape
    for shape in shapes {
        print_area(shape);
    }

}