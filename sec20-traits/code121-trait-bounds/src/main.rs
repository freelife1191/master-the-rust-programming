/*
`T`는 두 가지 제약 조건을 충족해야 하는 제네릭 유형이다. 즉, `Output = T`인 `std::ops::Mul` 특성을 구현해야 하며, Copy 특성을 구현해야 한다.

곱셈 연산이 피연산자와 동일한 유형을 반환하도록 하려면 `std::ops::Mul` trait bound `(T: std::ops::Mul<Output = T>)`에 `Output = T` 제약 조건이 필요하다.
 */
// impl<T: std::ops::Mul<Output = T>> Rectangle<T> {
impl<T> Rectangle<T>
    where T: std::ops::Mul<Output = T> + Copy + PartialOrd,
{

    fn area(self) -> T {
        self.width * self.height
    }
}

fn compare<T>(item1: T, item2: T) -> bool
    where T: PartialOrd,
{
    item1 > item2
}

fn combine<T, U>(item1: T, item2: U) -> (T, U)
    where T: std::fmt::Debug,
          U: std::fmt::Display,
{
    (item1, item2)
}

struct Rectangle<T> {
    width: T,
    height: T,
}

fn main() {
    let rect1 = Rectangle { width: 5.0, height: 10.0 };
    let rect2 = Rectangle { width: 7.5, height: 3.2 };

    println!("Area of rect1: {}", rect1.area());
    println!("Area of rect2: {}", rect2.area());
}