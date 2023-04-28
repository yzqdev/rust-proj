use std::fs;

fn main() {
    let pattern =  std::env::args().nth(1).expect("no pattern given");
    println!("{}",pattern+"hhh" );
    read_file();
    println!("Hello, world!");
    let mut str=String::from("hello");
    str.push_str("niuew");
    println!("{str}")
}

fn read_file(){
    let file_path="Cargo.toml";
    // --snip--
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}