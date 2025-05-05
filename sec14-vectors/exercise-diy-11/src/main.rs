#![allow(dead_code)]

/// 간단한 기상 관측소 애플리케이션 구축
/// 다양한 도시의 날씨 데이터를 기록하고 관리하는 것이 목표이다
/// 힌트: '벡터' 섹션에서 데모 비디오를 시청

enum WeatherCondition {
    //TODO
}

struct Weather {
    //TODO
}


struct CityWeather {
    //TODO
}

struct WeatherStation {
    //TODO
}

fn display_prompt() {
    let font_green = "\x1b[32m";
    let font_reset = "\x1b[0m";

    print!(
        r#"{}
\\\\\\Simple Weather Station\\\\\\\\\\
\\\\\\Display all weather reports -- 0
\\\\\\Add a new weather report    -- 1
\\\\\\Display city weather report -- 2
\\\\\\Update weather report       -- 3
\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
Enter your choice: {}"#,
        font_green, font_reset
    );

    stdout_flush();
}


fn display_all_weather_reports(weather_station: &WeatherStation) {
    todo!(); //remove this macro after implementing this function
}

fn get_new_weather_report_from_user() -> CityWeather {
    todo!();
}

fn add_new_weather_report(weather_station: &mut WeatherStation) {
    todo!();
}

fn display_city_weather_report(weather_station: &WeatherStation) {
    todo!();
}

fn update_city_weather_report(weather_station: &mut WeatherStation) {
    todo!();
}

fn main() {
    let mut weather_station: WeatherStation = WeatherStation::new();

    loop {
        display_prompt();

        match read_user_input() {
            0 => display_all_weather_reports(&weather_station),
            1 => add_new_weather_report(&mut weather_station),
            2 => display_city_weather_report(&weather_station),
            3 => update_city_weather_report(&mut weather_station),
            _ => {
                println!("Exiting...");
                break;
            }
        }

        print!("Press Enter key to continue...");
        stdout_flush();
        let _ = stdin_read_string();
    }
}

fn stdout_flush() {
    let _ = std::io::stdout().flush();
}

fn stdin_read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin");
    input.trim().to_string()
}

fn stdin_read_float() -> f32 {
    let input = stdin_read_string();
    input
        .parse::<f32>()
        .expect("Error while converting string to f32")
}

fn stdin_read_integer() -> u32 {
    let input = stdin_read_string();
    input
        .parse::<u32>()
        .expect("Error while converting string to u32")
}
