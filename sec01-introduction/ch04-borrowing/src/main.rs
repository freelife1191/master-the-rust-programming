fn main() {
    let mut x = 10;
    let y = &x; // const int *y = &x;
    // y = 15; // 오류: 'y'는 빌려왔기 때문에 할당할 수 없습니다.
    x = 15; // x = 15
    println!("{}", x);
}
