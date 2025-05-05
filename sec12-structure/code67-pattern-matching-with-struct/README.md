## Pattern matching using Struct

Destructuring a struct

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };

    match p {
        // Note : the arguments inside { .. }
        // must be field names of the struct
        Point { x, .. } => println!("x is {}", x),
    }
}
```

Matching multiple fields with guard

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };

    match rect {
        Rectangle { width: w, height: h } if w == h => {
            println!("The rectangle is square!");
        },
        Rectangle { width: _, height: _ } => {
            println!("The rectangle is not square.");
        },
    }
}
```

Matching against nested structs

```rust
#![allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 20, y: 0 },
    };

    match rect {
        Rectangle { top_left: Point { x: 0, .. }, .. } => println!("The top-left corner of the rectangle is on the x-axis."),
        Rectangle { .. } => println!("The rectangle is somewhere else."),
    }
}
```