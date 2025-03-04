use anyhow::{Error, Result};
use std::io;

fn read_file(path: &str) -> Result<()> {
    let _file = std::fs::File::open(path)?;
    Ok(())
}
#[test]
fn test_anyhow_app() {
    match read_file("nonexistent.txt") {
        Ok(_) => println!("Success!"),
        Err(e) => {
            if let Some(io_error) = e.downcast_ref::<io::Error>() {
                println!("IO error: {}", io_error);
            } else {
                println!("Other error: {}", e);
            }
        }
    }
}
