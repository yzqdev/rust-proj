use std::fs;
use std::path::Path;
use std::result::Result::Ok;
use std::{env::current_dir, fs::File};

#[test]
fn test_io_fs()   {
    let cwd = current_dir();
    match cwd {
        Ok(cur) => {
            println!("{:?}", cur)
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
    let f = File::open("Cargo.toml");
    let _r = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let path = Path::new("e:/tmpgit");
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
    let dir_files=path.read_dir().unwrap() ;
   let path = Path::new("./"); // 指定文件夹路径，这里是当前目录
    let entries = fs::read_dir(path).expect("error");

    // 使用 map 来遍历每个条目并打印文件名
    entries
        .filter_map(Result::ok) // 过滤掉读取错误的条目
        .map(|entry| {
            println!("{:?}",entry.path());
            entry.path()}) // 获取每个条目的路径
        .filter(|path| path.is_file()) // 只保留文件
        .for_each(|path| println!("{}", path.display())); // 打印文件路径
    
}
