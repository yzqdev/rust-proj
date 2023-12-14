#[allow(dead_code)]
use std::{fs::File, io::{ErrorKind, Read}};

pub fn create_file_if_not(){
       let f = File::open("test.txt");
    match f {
        Ok(file) => file,

        //再用match分析错误的类型
        Err(error) => match error.kind() {
            //不存在则创建
            //创建文件也有可能失败
            ErrorKind::NotFound => match File::create("dist/test.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建失败：{}", e),
            },
            others => panic!("打开失败（不能创建）：{}", others),
        },
    };
}
pub fn simple_open_file(){
    let f=File::open("dist/test.txt");
    match f {
        Ok(mut file) => {
            println!("文件打开成功");
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            println!("{}", content);
        },
        Err(error) => panic!("打开失败：{}", error),
    };
    let test_file=File::open("dist/test1.txt").expect("open file");
    println!("文件打开成功");

}