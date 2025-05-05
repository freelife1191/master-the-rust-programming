# package, create, cargo.toml, cargo.lock

### Rust 코드 구조화: Crates, Modules, Packages, Editions, 그리고 Versioning

## Package

* package는 함께 묶인 하나 이상의 crate 모음임
* `Cargo.toml`: 이것은 package의 설정 파일임 package에 대한 metadata (이름, 버전, 작성자 등), dependencies, 그리고 다른 설정들을 포함함

```bash
my_project/       # Cargo를 사용하여 생성하는 Rust project (또는 package)의 이름
├── Cargo.toml    # project를 위한 Manifest 파일
├── Cargo.lock    # dependency 버전을 고정하기 위해 자동으로 생성되는 파일
├── src/          # 소스 디렉토리
│   ├── lib.rs    # library crate를 위한 주 소스 파일
│   └── main.rs   # 기본 binary crate를 위한 주 소스 파일
└── bin/          # 추가적인 binary crate들을 위한 디렉토리
    ├── bin1.rs   # 다른 binary target을 위한 소스 파일
    └── bin2.rs   # 또 다른 binary target을 위한 소스 파일
```

### `cargo new <package-name>`

`cargo new <package-name>` command를 실행하면, `Cargo` 는 주어진 이름으로 새로운 `package` 를 생성함 기본적으로, `binary` `package` 를 생성하며, 이는 실행 가능한 프로그램을 생성하도록 설정되었음을 의미함

```bash
<package-name>.  # `<package-name>` 은 전체 `package` 를 포함하는 디렉토리임
├── Cargo.toml
├── Cargo.lock
└── src/
    └── main.rs
```

### `Cargo.toml` Vs `Cargo.lock`

* `Cargo.toml` 은 청사진과 같으며, `project` 에 필요한 것을 정의하는 곳임
* `Cargo.lock` 은 스냅샷으로, `project` 에서 사용되는 모든 것의 정확한 버전을 고정하여 다른 환경에서도 일관된 빌드를 보장함

## Crate

crate는 binary (실행 가능) 또는 library (재사용 가능 코드)로 컴파일될 수 있는 코드 `package` 임

* Binary crate
* Library crate

### Binary crate

* binary `crate` 는 실행 가능한 프로그램으로 컴파일됨 독립형 `application` 으로 실행되도록 의도됨 Rust의 빌드 시스템 (`Cargo`)을 사용하여 `binary` `crate` 를 빌드하면 실행 파일이 생성됨
* `binary` `crate` 의 주된 특징은 프로그램의 `entry point` 역할을 하는 `main` 함수를 포함한다는 것임
* Binary `crate` 는 커맨드 라인 `application` 이나 데스크톱 `application` 을 개발할 때 사용하는 것임

### Library crate

* `library` `crate` 는 실행 파일(executable)이 아닌 `library` 로 컴파일됨
* 일반적으로 `functions`, `structs`, `enums`, `Traits` 등과 같은 재사용 가능한 코드를 다른 `crate` 에 제공하는 데 사용됨
* `library` `crate` 는 `library` 의 루트(root) 역할을 하고 공개 `API` 를 정의하는 `lib.rs` 파일의 존재로 식별됨
* 다른 `project` 들은 이 `library` `crate` 들을 `dependencies` 로 사용하여 제공되는 코드와 기능을 재사용할 수 있음

* `cargo new my_binary_project`
    * 이 `command` 는 `binary` `crate` 에 필요한 파일들을 포함하는 `my_binary_project` 라는 새 디렉토리를 생성함
    * `src` 디렉토리가 생성되고 `main.rs` 파일을 포함할 것임
* `cargo new my_library_project --lib`
    * 이 `command` 는 `library` `crate` 를 위한 설정을 갖춘 `my_library_project` 라는 새 디렉토리를 생성함
    * `src` 디렉토리가 생성되지만, `main.rs` 파일 대신 `lib.rs` 파일을 찾을 수 있음 `lib.rs` 파일은 `library` 의 기능을 정의하는 곳임