/* break가 실행되면 루프가 즉시 종료되고 프로그램이 실행된다
   루프 이후의 명령문에서 계속 실행된다
*/
fn main() {
    let mut i = 0;

    loop {
        if i == 3 {
            break;
        }
        println!("i = {}", i);
        i += 1;
    }

    println!("loop ends");
}
