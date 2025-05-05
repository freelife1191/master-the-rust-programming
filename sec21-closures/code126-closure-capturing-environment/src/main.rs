fn main() {
    let mut x = 10;
    {
        // 클로저는 'x'를 가변적으로 빌려 수정합니다
        // let mut print = || {
        /*
        let mut print = move || {
            x += 1;
            println!("Modified x: {}", x);
        };
        */
        /*
        let mut print;

        {
            let mut x = 10;

            // let mut print = move || {
            print = move || {
                x += 1;
                println!("Modified x: {}", x);
            };
        }
        */
        /*
        let mut x = 10;

        let mut print = move || {
            x += 1;
            println!("Modified x: {}", x);
        };
        print(); // "Modified x: 11"
        print();
        println!("{}", x); // 10
        */
        let mut x = "10".to_string();

        let print = || {
            x += "11";
            println!("Modified x: {}", x);
            x
        };

        println!("{}", print()); // "Modified x: 1011"
        // println!("{}", print());
    }
    // 클로저를 여러 번 호출하고 'x'를 수정할 수 있습니다
}
