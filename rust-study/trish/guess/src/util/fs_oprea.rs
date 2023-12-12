use std::io;

use std::fs::File;
use std::io::Read;

pub fn get_file_text() -> Result<String, io::Error> {
    let mut f = File::open("./Cargo.toml")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    println!("{}", s);
    Ok(s)
}
