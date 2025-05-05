use std::fmt;

struct Dog {
    weight: u8,
    age: u8,
    name: String,
}

impl fmt::Display for Dog {
    // https://doc.rust-lang.org/stable/std/fmt/type.Result.html
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // https://doc.rust-lang.org/stable/std/fmt/struct.Formatter.html#method.width
        if let Some(w) = f.width() {
            writeln!(f, "{:#^w$}", "Dog details", w = 2 * w)?;
            writeln!(f, "{:<w$}:{:>w$}", "Age", self.age)?;
            writeln!(f, "{:<w$}:{:>w$}", "Weight", self.weight)?;
            writeln!(f, "{:<w$}:{:>w$}", "Name", self.name)?;
        } else {
            writeln!(f, "{:#^17}", "Dog details")?;
            writeln!(f, "{:<8}:{:>8}", "Age", self.age)?;
            writeln!(f, "{:<8}:{:>8}", "Weight", self.weight)?;
            writeln!(f, "{:<8}:{:>8}", "Name", self.name)?;
        }
        Ok(())
    }
}

fn main() {
    let my_dog = Dog {
        weight: 2,
        age: 2,
        name: "Bow Wow".to_string(),
    };

    println!("{:15}", my_dog);
    println!("{}", my_dog);
}