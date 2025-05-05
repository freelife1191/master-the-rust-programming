// #[derive(Default, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn new_with_defaults() -> Rectangle {
        Rectangle::default()
    }
    */

    // self는 메소드가 실행 중인 구조체의 인스턴스를 나타냅니다
    // called
    // Self는 구조체의 유형을 나타냅니다
    // methods/assoc. functions have been implemented
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    fn area(&self) -> f32 {
        (self.width * self.height) as f32
    }
}

fn main() {
    // println!("{:?}", Rectangle::new(10, 20));
    // println!("{:?}", Rectangle::new_with_defaults());
    let rect = Rectangle::new(10, 20);
    println!("area: {}", rect.area()); // 200
}