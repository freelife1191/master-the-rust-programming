# `Box<dyn Trait>`를 사용하여 `Vector`에 `heterogeneous data` `Storing`하기

## Vector에 trait objects 저장하기: Box<dyn Trait> 사용

`the same trait`를 `implement`하는 `different types`의 `objects`를 `hold`하기 위해 `Vec`가 필요한 상황을 고려해 보자

```rust
// `trait objects`의 `vec`
trait Shape {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Shape for Square {
    fn draw(&self) {
        println!("Drawing a square with side: {}", self.side);
    }
}

fn main() {
    // `trait objects`의 `vec`
    let shapes: Vec<dyn Shape> = vec![
        Circle { radius: 3.0 },
        Square { side: 2.0 }
    ];
}
```

```bash
Exited with status 101
Standard Error
Compiling playground v0.0.1 (/playground)
error[E0277]: the size for values of type `dyn Shape` cannot be known at compilation time
--> src/main.rs:28:17
  |
28 | let shapes: Vec<dyn Shape> = vec![
  |             ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
= help: the trait `Sized` is not implemented for `dyn Shape`
note: required by an implicit `Sized` bound in `Vec`
--> /playground/.rustup/.../alloc/vec.rs:397:8
  |
397 | pub struct Vec<T, # [unstable(feature = "allocator_api", issue = "32838")] A: Allocator = G
  |       ^ required by the implicit `Sized` requirement on this type parameter in `Vec`
error[E0308]: mismatched types
--> src/main.rs:29:9
  |
29 |         Circle { radius: 3.0 },
  |         ^^^^^^^^^^^^^^^^^^ expected `dyn Shape`, found struct `Circle`
  |
= note: expected trait object `dyn Shape`
         found struct `Circle`
```

```rust
fn main() {
    // `trait objects`의 `vec`
    let shapes: Vec<Box<dyn Shape>> = vec![
        // &Circle { radius: 3.0 } as &dyn Shape, //&Circle --> &dyn Shape
        Box::new(Circle { radius: 3.0 }), //&Circle --> &dyn Shape
        // &Square { side: 2.0 } as &dyn Shape //&Square --> &dyn Shape
        Box::new(Square { side: 2.0 }) //&Square --> &dyn Shape
    ];

    for shape in shapes {
        shape.draw();
    }
}
```

```bash
Drawing a circle with radius: 3
Drawing a square with side: 2
```