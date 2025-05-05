# 특수 문자가 포함된 텍스트 인쇄

```shell
David says, "Programming is fun!"
**Conditions apply**, "Offers valid until tomorrow"
C:\My computer\My folder
\\\\\\ Today is holiday \\\\\\
This is a triple quoted string """ This month has 30 days """
```

```rust
fn main() {
    /*
       특별한 내용이 포함된 메시지를 인쇄합니다.
       큰따옴표("blah blah") 및 백슬래시('\'와 같은 문자)
    */
    //println!("David says, "Programming is fun"");//Error
    println!("David says, \"Programming is fun\""); //OK. Note that \ used to help compiler escape ' " '

    //println!("C:\My computer\My folder");//Error
    println!("C:\\My computer\\My folder"); //OK. '\' used to help compiler escape '\'
}
```

Rust 에서는 인쇄 매크로와 이스케이프 문자(`\`)의 조합을 사용하여 큰따옴표나 역슬래시(`\`)와 같은 특수 문자가 포함된 문자열을 인쇄할 수 있습니다


## raw string example

이 예를 생각해 보세요. 여기서는 텍스트 줄 아래에 인쇄하려고 합니다

```shell
C:\My computer\My folder
```

> 인쇄하려는 모든 백슬래시마다 이스케이프 문자 `\`를 사용하는 대신 전체 문자열에 문자 `r`을 태그하면 문자열에서 이스케이프 시퀀스가 인식되지 않습니다

Solution 1 : 이스케이프 문자 `\`를 사용하세요.

`println!("C:\\My computer\\My folder");`

Solution 2 : 문자열에 'r' 문자를 태그로 지정하세요.

```rust
fn main() {
    /* 'r' 태그가 붙은 문자열에서 이스케이프 문자 '\'가 인식되지 않기 때문에 작동합니다 */
    println!(r"C:\My computer\My folder");
    let message = r"\ \ \ \ Today is holiday \ \ \ \";
    println!("{}", message);
}
```


## string tagging with `r#`

> `r` 태그가 붙은 문자열은 기본적으로 메시지에 있는 큰따옴표를 인식하지 못합니다

이 코드를 실행해 보세요

```
let message = r"\ \ \ \ "Today is holiday" \ \ \ \";
println!("{}", message);
```


Solution is to tag the string with r# and mark the end of tagging with #.


```rust
fn main() {
    /* string tagging with r#.......# */
    /* `####`은 가독성을 위해 사용됩니다. 원하는 만큼 `#s`를 사용할 수 있습니다. */
    let message = r####"\ \ \ \ "Today is holiday" \ \ \ \"####;
    let message = r#"\ \ \ \ "Today is holiday" \ \ \ \"#; /* same as above */
    println!("{}", message);
}
```

## 큰 텍스트 구절 인쇄

https://en.wikipedia.org/wiki/Rust_(programming_language)#Keywords_and_control_flow

