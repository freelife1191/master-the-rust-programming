fn main() {
    let mut fruits_data: [(&str, i32, i32); 7] = [
        ("Mango-us",50,80),
        ("Mango-uk",50,80),
        ("Orange",19,80),
        ("Blackberry",20,90),
        ("Blueberry",17,91),
        ("Blueberry",17,93),
        ("Blueberry",21,85),
    ];

    let len = fruits_data.len();

    //this loop is for number of passes through the list
    for _ in 0..len {
        //inner loop for the actual comparison and swapping of elements
        for i in 0..len-1 {
            if fruits_data[i + 1] < fruits_data[i] {
                fruits_data.swap(i + 1, i);
            }
        }
    }

    for value in fruits_data {
        println!("{:?}", value);
    }
}
