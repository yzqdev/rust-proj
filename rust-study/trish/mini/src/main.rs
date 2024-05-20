use mini::{files, json_util};

fn main() {
    println!("Hello, world!");
    files::file_control::get_all_lines();
    // read_lines();
    json_opera();
}
fn json_opera(){
    json_util::json_decode();
}