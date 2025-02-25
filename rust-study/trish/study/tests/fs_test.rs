use std::io::{self, Read};
use std::{env, fs, fs::File};
use std::net::IpAddr;
use std::path::Path;
use study::io_use::conf_constant::UNBUILD_CONF;
use study::io_use::simple_fs::add;

#[test]
fn it_works() {

    let result = add(2, 2);
    let cwd=env::current_dir();
    if let Ok(cur)=cwd {
        println!("{:?}",cur);
    }
    println!("{}",result);
    fs::write(Path::new("../target/build.config.ts"), UNBUILD_CONF)
        .expect("cant find target foldr");
    assert_eq!(result, 4);
}
#[test]

pub fn read_txt()->io::Result<()>{
    println!("{:?}",env::current_dir().unwrap());
      let greeting_file_result = File::open("Cargo.toml");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
     let mut buffer = String::new();

   
    greeting_file.read_to_string(&mut buffer)?;
    println!("{}",buffer);
    Ok(())
}

#[test]
fn say(){
    let s="herttt";
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("{home}");
}
#[test]
fn path_use(){
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new(".");

    // `display` 方法返回一个可显示（showable）的结构体
    let display = path.display();

    // `join` 使用操作系统特定的分隔符来合并路径到一个字节容器，并返回新的路径
    let new_path = path.join("a").join("b");

    // 将路径转换成一个字符串切片
    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence"),
        Some(s) => println!("new path is {}", s),
    }
}

#[test]
fn get_current_dir() -> std::io::Result<()>{
    let path = env::current_dir()?;
    let exe=env::current_exe();
    println!("{}",exe.expect("this is exe").display());
    println!("{}",path.display());

    Ok({})
}