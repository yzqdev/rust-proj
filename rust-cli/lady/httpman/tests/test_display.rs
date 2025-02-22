// （使用 `use`）导入 `fmt` 模块使 `fmt::Display` 可用
use std::{env, fmt};

// 定义一个结构体，咱们会为它实现 `fmt::Display`。以下是个简单的元组结构体
// `Structure`，包含一个 `i32` 元素。
struct Dog {
    name: String,
    value: String,
}
trait Animal {
    fn new(val: &str)->Self;
    fn say(&self);
    fn bite(&self);
}
 impl Dog {
     fn shoot(&self){
        println!("shoot at me");
     }
 }
impl Animal for Dog {
    fn say(&self) {
        print!("hello")
    }

    fn bite(&self) {
        println!("heeee")
    }
    
     fn new(val: &str) ->Self{
        Dog {
            name: val.to_string(),
            value: val.to_string(),
        }
    }
}

// 为了使用 `{}` 标记，必须手动为类型实现 `fmt::Display` trait。
impl fmt::Display for Dog {
    // 这个 trait 要求 `fmt` 使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将 self 的第一个元素写入到给定的输出流 `f`。返回 `fmt:Result`，此
        // 结果表明操作成功或失败。注意 `write!` 的用法和 `println!` 很相似。
        write!(f, "{}", self. name)
    }
}
#[test]
fn test_dp() {
    let a :Dog  = Animal::new("diplay");
    println!("{}", a);
    a.say();
    a.shoot();
    let is_win=cfg!(windows);
    println!("{}",is_win);
    println!("{}",env::var("android_proj").unwrap());
}
