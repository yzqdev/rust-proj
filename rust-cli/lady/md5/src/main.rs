

use std::{env, fs};
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use digest::Digest;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", &args[1]);
    let now = Instant::now();

    gen_fsmd5(&args[1]);
    println!("用时: {}s", now.elapsed().as_secs());

}
fn gen_fsmd5(file: &str){
    let mut buffer = [0u8;5];
    let mut read_file = fs::File::open(file).unwrap();

    loop {
        let read_res = read_file.read(&mut buffer).unwrap();
        // println!("==>{:?}==={:?}",read_res, buffer);
        // 清空多余的数据
        buffer = [0u8;5];
        if read_res < buffer.len() {
            break;
        }
    }

    println!("{:x}", md5::Md5::digest(buffer));
}
