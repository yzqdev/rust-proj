use std::{fs, io::Read};

use md5::Digest;


pub fn gen_fsmd5(file: String){
    let mut buffer = [0u8;8192];
    let mut read_file = fs::File::open(file).unwrap();

    loop {
        let read_res = read_file.read(&mut buffer).unwrap();
        buffer = [0u8;8192];
        if read_res < buffer.len() {
            break;
        }
    }

    println!("{:x}", md5::Md5::digest(buffer));
}
