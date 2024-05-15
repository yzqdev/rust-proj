mod io_use;

use md5::{Digest, Md5};
use std::fs::File;
use std::time::Instant;
use std::{env, fs, io};

use crate::io_use::simple_fs::add;
fn main() {
   let start=Instant::now();
   println!("{:?}",start);
   
}

fn get_file_md5(){
     let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);
    let now = Instant::now();
    let f = fs::read(&args[1]);
    println!("{:x}", md5::Md5::digest(f.unwrap()));
    println!("用时: {}s", now.elapsed().as_secs());
}