use crate::util::{
    fs_oprea::get_file_text,
    req::{self, main_req},
    str_fun::get_str_type,
};
use rand::Rng;
use std::{collections::HashMap, io};

fn simple_print() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("faile");
    println!("you guess {guess}");
    get_str_type(String::from("hhh"));
    let res = main_req();
    large_req();
    say();
    get_file_text().expect("TODO: panic message");
}

fn large_req() {
    let mut hash = HashMap::new();
    for i in 1..1000000 {
        hash.insert(String::from("k{i.to_string()}"), i);
    }
    println!("finish hash map")
}

fn say() {
    fn main_fun() -> String {
        let data = "initial contents";

        let _s: String = data.to_string();

        // 该方法也可直接用于字符串字面值：
        let _s = "initial contents".to_string();
        return _s;
    }

    let mut x: i32 = 44;
    x = 9934;
    let number: i32 = 3;
    if number > 4 {
        println!("hhh=>{x}")
    } else {
        println!("hhhbb")
    }
    for num in 1..10 {
        println!("{num},{}", main_fun())
    }
}
