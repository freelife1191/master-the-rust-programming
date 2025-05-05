# Section : `[dependencies]`

* 만약 `package` 가 `dependencies` (다른 `package` 들)를 가지고 있다면, `Cargo.toml` 에 명시되어야 함
* 각 `dependencies` 는 다른 `edition` 을 사용하고 있을 수 있음
* `Dependencies` 는 본질적으로 다른 `package` 들이며, 자신들의 `edition` 과 `dependencies` 를 명시하는 그들만의 `Cargo.toml` 파일을 가지고 있음 다른 `package` 에 의존할 때, `Cargo` 는 통합(integration)을 원활하게 관리하여 `edition` 간 호환성(inter-edition `compatibility`)을 허용함

```toml
[package]
name = "nyt_top_stories"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
reqwest = { version = "0.11.23", features = ["json"] }
tokio = { version = "1.35", features = ["full"] }
serde_json = "1.0.111"
```