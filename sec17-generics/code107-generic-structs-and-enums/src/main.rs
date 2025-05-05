/*
Generic with struct
 */

#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

/*
struct with 2 generic type parameters
 */
/*
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}
*/
/*
Methods of generic type struct
 */
/*
impl<T> Pair<T> {
    // fn new(first: i32, second: i32) -> Pair<i32> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    fn get_first(&self) -> T {
        &self.first
    }
}
*/
/*
Mix of generic and concrete methods
 */
impl<T: Copy> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    fn get_first(&self) -> T {
        self.first
    }
}

impl Pair<String> {
    fn get_first_string(self) -> String {
        self.first
    }
}

fn main() {
    // let pair_of_ints = Pair::<u8> { first: 1, second: 2 };
    let pair_of_ints = Pair { first: 1, second: 2 };
    // let pair_of_strings = Pair { first: "hello", second: "world" };
    let pair_of_strings = Pair { first: "hello".to_string(), second: "world".to_string() };
    // let pair_of_ints = Pair { first: 1, second: "hello" };
    // let pair_of_strings = Pair { first: "hello", second: 4.5 };
    println!("{:?}", pair_of_ints);
    println!("{:?}", pair_of_strings);

    println!("first int: {:?}", pair_of_ints.get_first());
    println!("first string: {:?}", pair_of_strings.get_first_string());
}
