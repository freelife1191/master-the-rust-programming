# Exercise-diy-6

학생들에게 문자 성적을 평가하고 할당하는 프로그램을 작성합니다  
다음을 사용하여 테스트 점수를 기반으로 합니다  

1. if-else-if ladder statement
2. match statement

Hint:  

다음 기준에 따라 문자 등급을 할당합니다  

```
90 to 100: Grade A
80 to 89: Grade B
70 to 79: Grade C
60 to 69: Grade D
Below 60: Grade F
```

Expected output:  

```
Enter student's test score: 85
The student's grade is: B
```

```rust
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
    
    //TODO
}
```