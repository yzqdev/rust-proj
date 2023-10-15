mod second_module;
mod files;

use crate::files::file_control::read_lines;

fn main() {
    println!("Hello, world!");
    files::file_control::get_all_lines();
    read_lines();
}
