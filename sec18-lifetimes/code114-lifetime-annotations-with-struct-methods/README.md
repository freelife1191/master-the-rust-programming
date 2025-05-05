## Lifetime annotations with struct methods

```rust
#[derive(Debug)]
struct MyStruct<'a, 'b> {
    data1: &'a str,
    data2: &'b str,
}
impl MyStruct {
    fn set_data1(&mut self, input: &str) {
        self.data1 = input;
    }
}
fn main() {
    /*
    Lifetime annotation for struct methods
     */
    let mut struct_ins = MyStruct {
        data1: "Hi",
        data2: "World",
    };

    struct_ins.set_data1("bye");
    println!("{:?}", struct_ins);
}
```

### Lifetime elision rule #3

입력 수명 매개변수가 여러 개 있지만 그 중 하나가 `&self` 또는 `&mut self`인 경우 `self`의 수명이 모든 출력 수명 매개변수에 할당된다