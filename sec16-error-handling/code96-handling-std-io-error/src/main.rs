use std::{fs, io};

// // https://doc.rust-lang.org/stable/std/io/type.Result.html
// fn rename_file(from: &str, to: &str) -> Result<(), io::Error> {
fn rename_file(from: &str, to: &str) -> io::Result<()> {
    // https://doc.rust-lang.org/stable/std/fs/index.html#functions
    // https://doc.rust-lang.org/stable/std/fs/fn.rename.html
    // std::fs::rename(from, to)
    /*
    match fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
    */
    fs::rename(from, to)?;
    Ok(())
}
fn main() {
    let res = rename_file("log.txt", "output.txt");
    if res.is_err() {
        let error = res.unwrap_err();
        println!("Rename failed");
        // https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html
        println!("Error kind : {:?}", error.kind());
        // https://doc.rust-lang.org/stable/std/io/struct.Error.html#method.kind
        println!("Error details : {}", error);
    } else {
        println!("Rename successful");
    }
}
