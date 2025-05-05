## Pattern matching with Enums

matching with wildcard patterns(`_`)

```rust
enum MyEnum {
    VariantA(i32, i32),
    VariantB(String),
    VariantC(bool, f64),
}

fn main() {
    let my_enum = MyEnum::VariantA(10, 20);

    match my_enum {
        MyEnum::VariantA(_, _) => println!("VariantA"),
        MyEnum::VariantB(_) => println!("VariantB"),
        _ => println!("Other variant"),
    }
}
```


```rust
enum MyEnum {
    VariantA(i32, i32, u8),
    VariantB(String),
    VariantC {is_ok: bool, x: f32, y: f64},
}

fn main() {
    let my_enum = MyEnum::VariantC {is_ok: false, x: 6.0, y: 1.0};

    match my_enum {
        MyEnum::VariantA(a, ..) => println!("VariantA with first field {}", a),
        MyEnum::VariantB(s) => println!("VariantB with value {}", s),
        MyEnum::VariantC {x, ..} => println!("VariantC with x field {}", x),
    }
}
```

.. 연산자를 사용하여 나머지 필드 일치