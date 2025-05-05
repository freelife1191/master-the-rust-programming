use std::io;
fn main() {
    let mut input = String::new();

    println!("Enter student's test score(Valid score: 0 to 100):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let score: u32 = input
        .trim()
        .parse()
        .expect("Invalid input");

    //동일한 로직을 구현하는 다양한 방법 탐색
    grade_score_using_if_else_if_without_range_expr(score);
    grade_score_using_if_else_if_with_range_expr(score);
    grade_score_using_match(score);
}

//if..else 사용 if..else + 논리 연산자 + 비교 연산자
fn grade_score_using_if_else_if_without_range_expr(score: u32) {
    if score < 60 {
        print_grade("F");
    } else if score >= 60 && score <= 69 {
        print_grade("D");
    } else if score >= 70 && score <= 79 {
        print_grade("C");
    } else if score >= 80 && score <= 89 {
        print_grade("B");
    } else if score >= 90 && score <= 100 {
        print_grade("A");
    } else {
        println!("Score cannt be higher than 100");
    }
}

//if..else 사용 if..else + 범위 표현식 + Iterator의 메소드 'contains'
fn grade_score_using_if_else_if_with_range_expr(score: u32) {
    if score < 60 {
        print_grade("F");
    } else if (60..=69).contains(&score) {
        print_grade("D");
    } else if (70..=79).contains(&score) {
        print_grade("C");
    } else if (80..=89).contains(&score) {
        print_grade("B");
    } else if (90..=100).contains(&score) {
        print_grade("A");
    } else {
        println!("Score cannt be higher than 100");
    }
}

//'matach' + 범위 표현식 사용
fn grade_score_using_match(score: u32) {
    match score {
        0..=59 => print_grade("F"),
        60..=69 => print_grade("D"),
        70..=79 => print_grade("C"),
        80..=89 => print_grade("B"),
        90..=100 => print_grade("A"),
        _ => println!("Score cannot be higher than 100"),
    }
}

//여기서 'grade' 인수는 &str 유형의 'String Slice'라고도 불리는 'String literal' 유형입니다.
fn print_grade(grade: &str) {
    println!("The Student's grade is : {grade}");
}
