/*
  'step_by'는 Rust의 Iterator 특성의 메서드이며 다음을 허용한다
  지정된 단계 크기로 범위를 반복한다
*/
// https://doc.rust-lang.org/std/primitive.array.html#impl-IntoIterator-for-%26%5BT;+N%5D
fn main() {
    for i in (0..100).step_by(2) {
        println!("{:2}", i);
    }
}