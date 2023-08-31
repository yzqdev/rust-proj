use md5::{Digest, Md5};
use std::fs::File;
use std::time::Instant;
use std::{env, fs, io};
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);
    let now = Instant::now();
    let f = fs::read(&args[1]);
    println!("{:x}", md5::Md5::digest(f.unwrap()));
    println!("用时: {}s", now.elapsed().as_secs());
}

// fn get_md5(){
//     let mut file = File::open("e:\\Downloads\\淘宝Lite_4.23.0.apk");
//     let mut hasher = Md5::new();
//     io::copy(&mut file, &mut hasher).expect("TODO: panic message");
//     let result = hasher.result();
//     println!("MD5 checksum of file.txt is: {:x}", result);
// }
