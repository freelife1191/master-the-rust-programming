struct Rectangle {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
}
enum Shape {
    Circle { x: f32, y: f32, radius: f32 },
    Rectangle(Rectangle),
    Square(f32, f32, f32),
}
impl Rectangle {
    fn new(x: f32, y: f32, h: f32, w: f32) -> Rectangle {
        Rectangle { x, y, h, w }
    }
}
impl Shape {
    fn new_circle(x: f32, y: f32, radius: f32) -> Self {
        Shape::Circle { x, y, radius }
    }
    fn new_rectangle(x: f32, y: f32, h: f32, w: f32) -> Self {
        Shape::Rectangle(Rectangle::new(x, y, h, w))
    }
    fn area(&self) -> f32 {
        match self {
            // https://doc.rust-lang.org/std/f32/consts/constant.PI.html
            Shape::Circle { radius: r, .. } => std::f32::consts::PI * r * r,
            Shape::Rectangle(rec) => rec.h * rec.w,
            Shape::Square(_, _, s) => s * s,
        }
    }
}
fn main() {
    // let new_shape = Shape::new_circle(0_f32, 2.0, 4.5);
    let new_shape = Shape::new_rectangle(0_f32, 2.0, 5.0, 4.0);
    let area = new_shape.area();
    println!("Area = {}", area);
}
