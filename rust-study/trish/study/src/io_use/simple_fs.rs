

pub fn add(x:i32,y:i32) -> i32{

    return x+y;
}
#[cfg(test)]
mod tests {
    use std::{fs, path::Path, process};

use super::*;

    #[test]
    fn it_works() {
 
        let result = add(2, 2);
        println!("{}",result);
        fs::write(Path::new("./target/a.txt"), "FROM RUST PROGRAM")
        .unwrap();
        assert_eq!(result, 4);
    }
}