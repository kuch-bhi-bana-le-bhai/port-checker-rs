use std::fs::File;
use std::io::Error;

fn openfile(filename: String) -> Result<File, Error> {
    let f = File::open(filename)?;
    Ok(f)
}

fn main() {
    let f = openfile(String::from("hello.txt"));

    // debug type of `f`
    println!("f={:?}", f);
}
