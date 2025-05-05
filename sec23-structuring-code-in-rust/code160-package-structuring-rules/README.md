# Package structuring rules

1. Rust `package` 는 최소 하나 이상의 `crate` 를 포함해야 하며, 이는 `library` 또는 `binary` `crate` 임
2. Rust `package` 는 여러 개의 `binary` `crate` 를 포함할 수 있지만, `library` `crate` 는 0개 또는 1개만 포함할 수 있음

* 만약 `package` 에 `src/lib.rs` 만 있다면, 하나의 `library` `crate` 를 가지며 `binary` `crate` 는 없음
* 만약 `package` 에 `src/main.rs` 만 있다면, 하나의 `binary` `crate` 를 가지며 `library` 는 없음
* 만약 `src/lib.rs` 와 `src/main.rs` 둘 다 있다면, 하나의 `library` `crate` 와 하나의 `binary` `crate` 를 가짐
* 만약 `src/bin` 디렉토리 아래에 `.rs` 파일을 더 추가하면, 각 파일은 `package` 내의 추가적인 `binary` `crate` 를 나타냄

```bash
weather_alert/
├── Cargo.toml
├── src/
│   ├── lib.rs    # 공통 날씨 `functions` 를 가진 `library` `crate`
│   ├── main.rs   # 날씨 예보를 위한 주 `application`
│   └── bin/
│       ├── report.rs # 날씨 리포트를 생성하는 `utility`
│       └── alert.rs  # 날씨 알림을 보내는 `utility`
```

## Crate Root file

Rust의 `project` `structure` 맥락에서, "root file" 이라는 용어는 `compiler` 가 `crate` 처리를 시작하는 주 소스 파일을 의미함

```bash
weather_alert/
├── Cargo.toml
├── src/
│   ├── lib.rs    # 공통 날씨 `functions` 를 포함하는 `library` `crate` 의 root 파일 역할을 함
│   ├── main.rs   # 이 `project` 의 주 `binary` `crate` 의 root 파일 역할을 함
│   └── bin/
│       ├── report.rs # 각각의 `binary` 에 대한 root 파일임
│       └── alert.rs  # 각각의 `binary` 에 대한 root 파일임
```

lib.rs

```rust
pub fn get_temperature() -> f32 {
    25.0 // Example temperature value in Celsius
}
```

alert.rs

```rust
use weather_report::get_temperature;

fn main() {
    let temp = get_temperature(); // get_temperature() 함수 정의가 필요합니다
    if temp > 30.0 {
        println!("Weather Alert: Temperature above 30°C!");
    } else {
        println!("Weather is normal.");
    }
}
```

report.rs

```rust
use weather_report::get_temperature; // weather_report 모듈 또는 crate가 필요합니다

fn main() {
    let temp = get_temperature();
    println!("Weather Report:");
    println!("Temperature: {}°C", temp);
    println!("Conditions: Sunny");
}
```

main.rs

```rust
use weather_report::get_temperature;

fn main() {
    let temp = get_temperature();
    println!("Current temperature: {}°C", temp);
}
```

Cargo.toml

```toml
[package]
name = "weather_report"
version = "0.1.0"
edition = "2021"
default-run = "weather_report"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

[[bin]]
name = "weather_app"
path = "src/main.rs"

[[bin]]
name = "report_app"
path = "src/bin/report.rs"

[[bin]]
name = "alert_app"
path = "src/bin/alert.rs"
```

## Summary

* `src/lib.rs` 는 `library` `crate` 의 `root` 이며 `src` `directory` 에 위치해야 함
* `src/main.rs` 는 `package` 의 주 `binary` `crate` 의 `root` 임
* `src/bin/*.rs` 는 별도의 추가 `binary` `crate` 들을 포함함 이 `directory` 안의 각 `.rs` 파일은 별도의 `binary` `executable` 로 컴파일됨