use std::{env, fs};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use digest::Digest;




fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);
    let now = Instant::now();
    // gen_md5(&args[1]);
    gen_fsmd5(&args[1]);
    println!("执行时间: {}s", now.elapsed().as_secs());

}
fn gen_fsmd5(file: &str){
    let mut f=fs::read(file).unwrap();
    println!("{:x}", md5::Md5::digest(f));
}

fn gen_md5(file: &str){
    //caculate md5 for file
     let mut f = File::open(file).unwrap();
    let mut buffer = Vec::new();
    // // read the whole file
    f.read_to_end(&mut buffer).unwrap();



    println!("{:x}", md5::Md5::digest(buffer));

}