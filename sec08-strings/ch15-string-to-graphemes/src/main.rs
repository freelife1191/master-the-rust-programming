use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let computer_in_hindi = "कंप्यूटर";
    println!("Hindi word: {}", computer_in_hindi);


    // 문자열의 모든 문자를 인쇄 ( 각 문자는 UTF 인코딩의 3바이트를 사용하여 인코딩 됨)
    print!("All characters of the string: "); // क  ं  प  ्  य  ू  ट  र
    for ch in computer_in_hindi.chars() {
        print!("{}",ch);
        print!("  ");
    }

    println!();

    // 문자열의 문자 수를 인쇄
    println!("Total chars : {}",computer_in_hindi.chars().count());

    // 문자열을 바이트 배열로 변환
    let byte_array = computer_in_hindi.as_bytes();
    println!("Byte array: {:?}", byte_array);

    // 문자열을 문자소 클러스터 반복자로 변환하고 인쇄합니다.
    let graphemes = computer_in_hindi.graphemes(true);
    for grapheme in graphemes {
        println!("Grapheme: {}", grapheme);
    }
}