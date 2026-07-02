use std::fs;
use std::io::Read;
use digest::Digest;

pub fn gen_fsmd5(file: &str) {
    let mut file = match fs::File::open(file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: cannot open '{}': {}", file, e);
            return;
        }
    };

    let mut hasher = md5::Md5::new();
    let mut buffer = [0u8; 8192];

    loop {
        let n = match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error: read failed: {}", e);
                return;
            }
        };
        hasher.update(&buffer[..n]);
    }

    println!("{:x}", hasher.finalize());
}
