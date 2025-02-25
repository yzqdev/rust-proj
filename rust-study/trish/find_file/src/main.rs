use clap::{Command, arg};
use find_file::{Config, main_fun};
use glob::glob;

fn main() {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .arg_required_else_help(true)
        .subcommand(Command::new("hostname").about("show hostname part of FQDN"))
        .subcommand(Command::new("gen"))
        .subcommand(
            Command::new("png")
                .about("find png")
                .arg(arg!(<PATH> "find png path")),
        );

    match cmd.get_matches().subcommand() {
        Some(("hostname", _)) => {
            let conf = Config {
                query: "todo!()".to_string(),
                file_path: r"D:\sciter-js-sdk-main\README.md".to_string(),
            };
            main_fun(conf);
        }
        Some(("png", png_match)) => {
            let path = png_match.get_one::<String>("PATH").expect("parser error");
            for entry in glob(format!("{}/**/*.png", path).as_str()).unwrap() {
                println!("{}", entry.unwrap().display());
            }
            println!("{}", env!("PATH"))
        }
        Some(("gen", _)) => {
            println!("gen")
        }

        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
