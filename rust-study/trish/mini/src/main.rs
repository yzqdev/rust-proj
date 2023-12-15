mod json_util;
mod files;
mod advance;

use crate::files::file_control::read_lines;

fn main() {
    println!("Hello, world!");
    files::file_control::get_all_lines();
    // read_lines();
    json_opera();
}
fn json_opera(){
    json_util::json_decode();
}