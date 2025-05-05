/*
   배열에 음수가 포함되어 있는지 확인하고 플래그를 업데이트합니다.
   그에 따라 변수 'invalid_array'가 필요합니다."
*/
fn main() {
    let array1 = [1, -2, 3, 4];

    /*
    let invalid_array = match array1 {
        [n, _, _, _] | [_, n, _, _] |
        [_, _, n, _] | [_, _, _, n] if n < 0 => {
            true
        }

        _ => false,
    };
    */

    // Clippy 에서 matches! 매크로를 사용하는 것이 더 좋을 거 같다고 제안
    let invalid_array = matches!(array1, [n,_,_,_] | [_,n,_,_] | [_,_,n,_] | [_,_,_,n] if n < 0);

    if invalid_array {
        println!("Array is invalid");
    } else {
        println!("Array is valid");
    }
}
