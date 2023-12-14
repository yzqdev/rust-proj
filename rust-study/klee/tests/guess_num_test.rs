use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("猜数字游戏");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入一个数字！");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败！");

        let guess: i32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("输入不正确，请输入数字！");
                continue;
            }
        };

        println!("你猜的数字是：{guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("太小了。。。。"),
            Ordering::Greater => println!("太大了。。。。"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}