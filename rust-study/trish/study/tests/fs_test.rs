use std::{env, fs::File};
use std::net::IpAddr;


#[test]

pub fn read_txt(){
    println!("{:?}",env::current_dir().unwrap());
      let greeting_file_result = File::open("Cargo.toml");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

#[test]
fn say(){
    let s="herttt";
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{home}");
}