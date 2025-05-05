
## Standard library error types

사용되는 프로그래밍 언어에 관계없이 코딩하는 동안 일반적으로 발생하는 여러 유형의 오류가 있습니다

- I/O Errors
  - File not found errors
  - Permission denied errors
  - Connection reset by peer errors (in the case of network I/O)
  - Timeout errors
  - Protocol errors
  - DNS lookup fails
- Numeric and Conversion Errors
  - Parsing Errors
  - Reading data from files or network streams
  - Converting between numeric types
- Formatting and data validation Errors
  - Converting a String from a UTF-8 byte vector
  - Formatting a message into a stream
- Network Errors
  - Parsing an IP address or a socket address


### documents

- https://doc.rust-lang.org/stable/std/io/struct.Error.html
- https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html
- https://doc.rust-lang.org/stable/std/num/struct.ParseIntError.html
- https://doc.rust-lang.org/stable/std/net/struct.AddrParseError.html