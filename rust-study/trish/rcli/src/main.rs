mod util;

use std::fs;
use std::time::SystemTime;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    println!("{}", pattern + "hhh");

    println!("Hello, world!");
    let mut str = String::from("hello");
    str.push_str("niuew");
    println!("{str}");
    util::file_opera::read_file()
}

mod c
{
    pub fn c()
    {
        println!("C is a structured programming language");
    }
}

mod cplus
{
    pub fn cplus()
    {
        println!("C++ is an object-oriented programming language");
    }
}

fn modules()
{
    c::c();
    cplus::cplus();
}
//更多请阅读：https://www.yiibai.com/rust/rust-modules.html

