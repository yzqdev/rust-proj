mod io_use;
mod macro_use;
mod trait_use;

use std::time::Instant;

use clap::{Arg, Command, arg};

use crate::io_use::simple_fs::{add, get_file_md5, file_size, list_dir};

fn main() {
    let m = Command::new("study")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Study tool - file operations, string utils, and more")
        .arg(Arg::new("in_file"))
        .subcommand(
            Command::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("md5")
                .about("Calculate MD5 hash of a file")
                .arg(arg!(<FILE> "file path")),
        )
        .subcommand(
            Command::new("size")
                .about("Get file size")
                .arg(arg!(<FILE> "file path")),
        )
        .subcommand(
            Command::new("ls")
                .about("List directory contents")
                .arg(arg!(<DIR> "directory path").default_value(".")),
        )
        .subcommand(
            Command::new("add")
                .about("Add two numbers")
                .arg(arg!(<A> "first number"))
                .arg(arg!(<B> "second number")),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();

    match m.subcommand() {
        Some(("clone", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("md5", sub_matches)) => {
            let file = sub_matches.get_one::<String>("FILE").expect("required");
            match study::io_use::simple_fs::compute_md5(file) {
                Ok(hash) => println!("MD5({}) = {}", file, hash),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Some(("size", sub_matches)) => {
            let file = sub_matches.get_one::<String>("FILE").expect("required");
            file_size(file);
        }
        Some(("ls", sub_matches)) => {
            let dir = sub_matches.get_one::<String>("DIR").expect("required");
            list_dir(dir);
        }
        Some(("add", sub_matches)) => {
            let a: i32 = sub_matches
                .get_one::<String>("A")
                .expect("required")
                .parse()
                .expect("A must be a number");
            let b: i32 = sub_matches
                .get_one::<String>("B")
                .expect("required")
                .parse()
                .expect("B must be a number");
            println!("{} + {} = {}", a, b, add(a, b));
        }
        _ => unreachable!(),
    }
}
