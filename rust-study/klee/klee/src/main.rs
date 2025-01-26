
 

use klee::basic::{file_opera::simple_open_file, struct_fun, trait_fun};

fn main() {
    println!("Hello, world!");

    // file_opera_fun();

    //stuct operation
    // struct_opera();
    trat_fun();
 
}
pub fn struct_opera(){
    struct_fun::struct_pear();
}
pub fn trat_fun(){
    trait_fun::trait_main();
}
pub fn file_opera_fun(){
    simple_open_file();
}