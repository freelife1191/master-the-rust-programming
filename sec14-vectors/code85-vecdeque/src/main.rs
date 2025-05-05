use std::collections::VecDeque;

fn main() {
    /*
    VecDeque<T>
    */
    let mut vd = VecDeque::with_capacity(6);
    vd.push_back(1);
    vd.push_back(2);
    vd.push_back(3);
    vd.push_front(4);
    vd.push_front(5);

    // https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.as_slices
    let (front, back) = vd.as_slices();

    println!("vd: {:?}", vd); // prints: [5, 4, 1, 2, 3]
    // println!("vd: {:?}", &vd[0..2]); // prints: [5, 4]
    println!("vd: {:?}", vd[3]); // prints: 2
    println!("front: {:?}", front); // prints: [5, 4]
    println!("back: {:?}", back); // prints: [1, 2, 3]

    // https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.make_contiguous
    let vd_slice: &mut [i32] = vd.make_contiguous();
    let (front, back) = vd.as_slices();
    println!("front: {:?}", front); // prints: [5, 4, 1, 2, 3]
    println!("back: {:?}", back); // prints: []

    /*
    VecDeque<T> vs Vec<T>
    1. 디자인: VecDeque는 양방향 큐로 설계되었습니다.
    2. 삽입 효율성: VecDeque는 앞면과 뒷면 모두에서 효율적인 삽입을 지원하고 Vec는 뒷면에서 효율적인 삽입을 지원한다
    3. 제거 효율성: VecDeque는 앞면과 뒷면 모두에서 효율적인 제거를 지원하고, Vec은 뒷면에서 효율적인 제거를 지원한다
    */
}
