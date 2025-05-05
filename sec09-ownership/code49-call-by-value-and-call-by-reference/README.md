# Call by value and Call by reference

- **Call by value**은 원래 값을 변경하지 않고 인수를 복사하여 함수에 전달하는 함수 인수 전달 방법입니다
- **Call by reference**은 복사본을 만드는 대신 변수에 대한 참조가 함수에 전달되는 함수 인수 전달 방법입니다  
  이를 통해 함수는 유용한 값 자체를 생성하지 않고도 원래 값에 직접 액세스하고 수정할 수 있습니다
- 참조에 의한 호출은 함수에 전달되는 차용의 소유권을 주장하지 않고 대신 값에 대한 참조를 취하므로 소유권을 전달하지 않고 일시적으로 유지하면서 함수가 값을 계산할 수 있습니다


```rust
fn main() {
    let arr = [10, 20, 30, 40, 50];

    // Call by value
    let max_value = find_greatest_value_by_value(arr);
    println!("Maximum value by value: {}", max_value);

    // Call by reference
    let max_value = find_greatest_value_by_reference(&arr);
    println!("Maximum value by reference: {}", max_value);
}

// Call by value
fn find_greatest_value_by_value(arr: [i32; 5]) -> i32 {
    // TODO
}

// Call by reference
fn find_greatest_value_by_reference(arr: &[i32]) -> i32 {
    // TODO
}
```


## References to References

```rust
fn main() {
    /*
      References to References
     */
    let x = 42;
    let x_ref_1 = &x; //type of 'x_ref_1' : &i32 참조
    let x_ref_2 = &x_ref_1; //type of 'x_ref_2' : &&i32 참조에 참조
    let x_ref_3 = &x_ref_2; //type of 'x_ref_3' : &&&i32 참조에 참조에 참조

    /*
    Rust는 `println!`과 같은 형식 지정 매크로에서 `{}` 지정자를 사용할 때 참조 유형 `T`가 Display 특성을 구현하는 경우 기본 값에 대한 참조(&T)를 자동으로 역참조합니다
     */
    println!("Value of x using 1 level of reference: {}", x_ref_1);
    println!("Value of x using 2 levels of reference: {}", x_ref_2);
    println!("Value of x using 3 levels of reference: {}", x_ref_3);

    // println!("Value of x using 1 level of reference: {}", *x_ref_1);
    // println!("Value of x using 2 levels of reference: {}", **x_ref_2);
    // println!("Value of x using 3 levels of reference: {}", ***x_ref_3);
}
```

## Comparing references

비교 연산자는 참조 자체의 메모리 주소를 비교하는 대신 참조가 가리키는 값을 비교합니다

```rust
fn main() {
    let x = 42;
    let x_ref = &x; //type of 'x_ref' : &i32
    
    let y = 42;
    let y_ref = &y; //type of 'y_ref' : &i32

    // 값은 내용에 따라 비교된다
    // 그리고 두 참조(==,>,<,<=,>=,!=,etc)를 비교할 때 Rust는 비교한다
    // 메모리 주소가 아닌 그들이 가리키는 값
    // 따라서 이 경우 조건은 true로 평가된다 포인터나 참조의 비교가 아니다
    if x_ref == y_ref {
        println!("values are the same");
    }

    /*
    if *x_ref == *y_ref {
        println!("values are the same");
    }
    */
}
```


```rust
fn main() {
    let x = 42;
    let x_ref_1 = &x; //type of 'x_ref_1' : &i32
    let x_ref_2 = &x_ref_1; //type of 'x_ref_2' : &&i32

    // This is Erro
    if x_ref_1 == x_ref_2 {
        println!("values are the same");
    }

    // This is OK
    if x_ref_1 == (*x_ref_2) {
        println!("values are the same");
    }
    
}
```