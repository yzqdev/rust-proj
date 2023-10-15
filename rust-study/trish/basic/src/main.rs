mod datatype;
use crate::datatype::{array_data::get_array, struct_data::Site};
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    let int_data = 111;
    println!("struct data {:?}", runoob);
    println!("helll{0}", int_data);
    println!("Hello, world!");
    get_array();
}
