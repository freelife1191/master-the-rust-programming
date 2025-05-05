/*
enum LightState {
    On { brightness: u8 },
    Off,
}
*/

enum MyEnum {
    VariantA(i32, i32, u8),
    // VariantA(i32, i32),
    VariantB(String),
    // VariantB(&str),
    VariantC {is_ok: bool, x: f32, y: f64},
    // VariantC(bool, f64),
}

fn main() {
    // let bulb = LightState::On { brightness: 200 };

    /*
    match bulb {
        LightState::On { brightness: 180 } => {
            println!("Brightness is 180");
        }

        LightState::On { brightness } => {
            println!("Light state is On with brightness: {}", brightness);
        }

        LightState::Off => println!("Light state is Off"),
    }
    */
    /*
    if let LightState::On { brightness: 200 } = bulb {
        println!("Brightness is 200");
    } else {
        println!("Brightness other than 200");
    }
    */
    let my_enum = MyEnum::VariantC {is_ok: false, x: 6.0, y: 1.0};
    // let my_enum = MyEnum::VariantA(10, 20);
    // let my_enum = MyEnum::VariantB("Sunday");
    // let my_enum = MyEnum::VariantB("Sunday".to_string());
    // let my_enum = MyEnum::VariantB("Monday".to_string());

    match my_enum {
        // .. 연산은 나머지 필드를 무시하고 나머지 필드를 무시하고 패턴 매칭을 수행한다
        MyEnum::VariantA(a, ..) => println!("VariantA with first field {}", a),
        // MyEnum::VariantA(_, 20) => println!("VariantA"),
        // MyEnum::VariantA(_, a @ 20) => println!("VariantA {}", a),
        MyEnum::VariantB(s) => println!("VariantB with value {}", s),
        // MyEnum::VariantB(_) => println!("VariantB"),
        // MyEnum::VariantB(s @ "Sunday") => println!("VariantB is Sunday"),
        // MyEnum::VariantB(s) if s == "Sunday" => println!("VariantB is Sunday"),
        MyEnum::VariantC {x, ..} => println!("VariantC with x field {}", x),
        _ => println!("Other variant"),
    }
}