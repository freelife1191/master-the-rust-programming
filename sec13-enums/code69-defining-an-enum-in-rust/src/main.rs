// #[derive(PartialEq)] // PartialEq 특성을 자동으로 파생하도록 요청
#![allow(unused_variables)] // 사용하지 않는 변수에 대한 경고 끄기
// creating an instance of the enum
struct Point {
    x: i32,
    y: i32,
}

// 해당 특성을 자동으로 파생하도록 요청
enum CarStatus {
    MovingUp(u32, i32, i32),
    MovingDown { speed: u32 },
    NotMoving(Point),
    NotWorking,
}

fn main() {
    let mut current_car_status = CarStatus::NotMoving(Point { x: 0, y: 0 });

    current_car_status = CarStatus::MovingUp(100, 67, 78);

    let another_car_status = CarStatus::NotMoving(Point { x: 10, y: 0 });

    // if Car is movingUP, print its speed
    /*
    if current_car_status == CarStatus::NotWorking {
        println!("Car is not working");
    }
    */
    // if Car is NotMoving then prints its x co-ordinate value
    match current_car_status {
        CarStatus::MovingUp(a, ..) => {
            println!("Car is moving up with speed: {}", a);
        }
        CarStatus::NotMoving(Point { x, .. }) => {
            println!("Car is not moving and its x co-ordinate is: {}", x);
        }
        _ => println!("Car is not moving up"),
    }
}
