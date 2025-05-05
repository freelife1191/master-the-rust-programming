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
fn print_area(shape: &dyn Shape) {
    println!("Area: {}", shape.area());
}


fn main() {
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle { width: 4.0, height: 5.0 };

    // area 메서드를 호출하려면 `trait objects`를 사용하세요.
    // `&Circle`과 `&Rectangle`은 모두 `&dyn Shape`로 강제될 수 있습니다.
    // 여기서 `vector`는 `trait objects`를 차용합니다.
    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];
    // 여기서 `vector`는 `trait objects`를 소유합니다.
    // let shapes: Vec<Box<dyn Shape>> = vec![Box::new(circle), Box::new(rectangle)];

    // type of 'shape' is &dyn Shape
    for shape in shapes {
        print_area(shape);
    }

    println!("{}", circle.radius);
}