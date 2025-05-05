/**
    Rust assert macros
    https://doc.rust-lang.org/std/macro.assert.html

    Standard Library Assert Macros
    - assert!(expression)
    - assert_eq!(left, right)
    - assert_ne!(left, right)
    - debug_assert!(expression)
    - debug_assert_eq!(left, right)
    - debug_assert_ne!(left, right)
    - assert!(expression, "message")
    - assert_eq!(left, right, "message")
    - assert_ne!(left, right, "message")
    - debug_assert!(expression, "message")
    - debug_assert_eq!(left, right, "message")
    - debug_assert_ne!(left, right, "message")
    - #[should_panic(expected = "panic message")]
**/

fn main() {
    // let x = false;
    // debug_assert!(x, "x wasn't true!");
    // --release profile 로 실행하면 Hi 메세지만 보인다
    // println!("Hi");

    let x = 2;
    assert_eq!(x, 1 + 2);

}
