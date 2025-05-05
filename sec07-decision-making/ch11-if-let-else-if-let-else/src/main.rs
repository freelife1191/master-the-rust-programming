//if let .. else if let
fn main() {
    let point = (4, 2);

    // if let 문은 match 문에 대한 약식 문법
    if let (0, 0) = point {
        println!("Point is at the origin");
        // 첫 번째 요소는 무시 두번째 요소인 y 가 1 ~ 4 범위 내에 있으면 조건 충족
    } else if let (_, y @ 1..=4) = point {
        println!("{} is within the range 1..4", y);
    } else {
        println!("something else");
    }
}