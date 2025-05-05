fn main() {
    /*
       특별한 내용이 포함된 메시지를 인쇄합니다.
       큰따옴표("blah blah") 및 백슬래시('\'와 같은 문자)
    */
    //println!("David says, "Programming is fun"");//Error
    println!("David says, \"Programming is fun\""); //OK. Note that \ used to help compiler escape ' " '

    //println!("C:\My computer\My folder");//Error
    println!("C:\\My computer\\My folder"); //OK. '\' used to help compiler escape '\'

    //
    // raw string example
    //
    /* 'r' 태그가 붙은 문자열에서 이스케이프 문자 '\'가 인식되지 않기 때문에 작동합니다 */
    println!(r"C:\My computer\My folder");
    // let message = r"\ \ \ \ "Today is holiday" \ \ \ \"; // Error
    // let message = r"\ \ \ \ \"Today is holiday\" \ \ \ \"; // Error
    let message_1 = r"\ \ \ \ Today is holiday \ \ \ \";
    println!("{}", message_1);

    /* Error. 'r' 태그가 지정된 문자열은 큰따옴표를 포함할 수 없기 때문에 */
    //println!(r"This is a triple quoted string """ This month has 30 days """  ");

    /* 'r' 태그된 문자열 이스케이프 문자 '\'가 인식되지 않아 오류 */
    //println!(r"This is a triple quoted string \"\"\" This month has 30 days \"\"\"  ");

    //
    //string tagging with r#.......#
    //
    /* ####은 가독성을 위해 사용됩니다. 원하는 만큼 #을 사용할 수 있습니다. */
    // let message = r#"\ \ \ \ "Today is holiday" \ \ \ \"#; // OK
    let message_2 = r####"\ \ \ \ "Today is holiday" \ \ \ \"####;
    let _message_3 = r#"\ \ \ \ "Today is holiday" \ \ \ \"#; /* same as above */
    println!("{}", message_2);

    /*
    큰 텍스트 구절 인쇄
     */
    // r####" your string goes here "####;
    let text = r####"
    fn main() {
        let value = 456;
        let mut x = 1;
        let y = loop {
            x *= 10;
            if x > value {
                break x / 10;
            }
        };
        println!("largest power of ten that is smaller than value: {y}");

        let mut up = 1;
        'outer: loop {
            let mut down = 120;
            loop {
                if up > 100 {
                    break 'outer;
                }

                if down < 4 {
                    break;
                }

                down /= 2;
                up += 1;
                println!("up: {up}, down: {down}");
            }
            up *= 2;
        }
    }
    "####;
    println!("{}", text);
}
