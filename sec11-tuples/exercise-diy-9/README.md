# Exercise-diy-9

(과일 이름, 가격, 수량) 튜플을 다음과 같이 정렬하는 프로그램을 작성해야 합니다
과일 이름은 문자열이고 가격과 수량은 숫자인 오름차순입니다

1: 과일 이름을 기준으로 정렬합니다.  
2: 그런 다음 가격을 기준으로 정렬합니다.  
3: 그런 다음 수량별로 정렬합니다.  
우선순위는 과일명 > 가격 > 수량 입니다.  

다음 튜플이 프로그램에 대한 입력으로 제공되는 경우:

```
Mango-us,50,80
Mango-uk,50,80
Orange,19,80
Blackberry,20,90
Blueberry,17,91
Blueberry,17,93
Blueberry,21,85
```

**Expected output:**

```
("Blackberry", 20, 90)
("Blueberry", 17, 91)
("Blueberry", 17, 93)
("Blueberry", 21, 85)
("Mango-uk", 50, 80)
("Mango-us", 50, 80)
("Orange", 19, 80)
```

Hint :
1. 튜플을 사전순으로 비교할 수 있다는 것을 기억하세요
2. 버블 정렬을 사용하세요. 논리는 코드에 제공됩니다  
   i번째 튜플을 (i+1)번째 튜플과 비교하고 필요한 경우 교환해야 합니다
3. 배열 요소를 교환하려면 `swap()` 메서드를 사용하세요 (https://doc.rust-lang.org/std/primitive.slice.html#method.swap)

```rust
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
```