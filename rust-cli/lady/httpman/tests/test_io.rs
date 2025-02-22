use std::fs;
use std::io::Read;
use std::path::Path;
use std::result::Result::Ok;
use std::{env::current_dir, fs::File};

#[test]
fn test_io_fs() {
    let cwd = current_dir();
    match cwd {
        Ok(cur) => {
            println!("{:?}", cur)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
    let f = File::open("Cargo.toml");
    let _r = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let path = Path::new("e:/tmpgit");
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
}
