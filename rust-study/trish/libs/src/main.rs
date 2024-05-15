pub mod use_trait;
 use std::fs::File;
use std::{env, io};
use rand::Rng ;
fn main() {
   let a = [10, 20, 30, 40, 50];
   for item  in 1..2   {
       println!("{}",item)
   }
   println!("{:?}",env::home_dir() );
   use_trait::use_trait();
   file_opera();
}



fn file_opera() {
    let f = File::open("hello.txt").unwrap();

     
}