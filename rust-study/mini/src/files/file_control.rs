
use std::io::stdin;

pub fn read_lines() {
    let mut str_buf = String::new();

    stdin().read_line(&mut str_buf)
        .expect("Failed to read line.");

    println!("Your input line is \n{}", str_buf);
}

pub fn get_all_lines(){
    println!("niuwa")
}