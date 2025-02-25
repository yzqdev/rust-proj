pub mod util;


use std::env;
use std::fs;
use std::process;

pub fn main_fun(conf:Config) {
    

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(conf).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

pub struct Config {
   pub query: String,
    pub file_path: String,
}

impl Config {
    fn build(args: Self) -> Result<Config, &'static str> {
       

        let query = args.query.clone();
        let file_path = args.file_path.clone();

        Ok(Config { query, file_path })
    }
}
