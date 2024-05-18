use std::{env, fs};
use std::time::Instant;
use md5::Digest;

pub fn get_file_md5() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", &args[1]);
        let now = Instant::now();
        let f = fs::read(&args[1]);
        println!("{:x}", md5::Md5::digest(f.unwrap()));
        println!("用时: {}s", now.elapsed().as_secs());
    }
}

pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path, process};
    use crate::io_use::conf_constant::UNBUILD_CONF;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        println!("{}", result);
        fs::write(Path::new("./target/build.config.ts"), UNBUILD_CONF)
            .expect("cant find target foldr");
        assert_eq!(result, 4);
    }
}