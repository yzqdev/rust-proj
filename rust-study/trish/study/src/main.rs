mod io_use;

use std::time::Instant;


use crate::io_use::simple_fs::{add, get_file_md5};
fn main() {

   get_file_md5();
   println!("{}",dirs::cache_dir().expect("power").display());
}

