use clap::{Arg, Command, arg};
use mini::{files, json_util, string_util, path_util};

fn main() {
    let matches = Command::new("mini")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Mini utility tool - file, json, string and path operations")
        .arg(Arg::new("in_file"))
        .subcommand(
            Command::new("gen")
                .about("Run generation demo (file ops + json)")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("string")
                .about("String utilities")
                .subcommand(
                    Command::new("reverse")
                        .about("Reverse a string")
                        .arg(arg!(<TEXT> "text to reverse")),
                )
                .subcommand(
                    Command::new("count")
                        .about("Count words in a string")
                        .arg(arg!(<TEXT> "text to count")),
                ),
        )
        .subcommand(
            Command::new("path")
                .about("Path utilities")
                .subcommand(
                    Command::new("ext")
                        .about("Get file extension")
                        .arg(arg!(<PATH> "file path")),
                )
                .subcommand(
                    Command::new("parent")
                        .about("Get parent directory")
                        .arg(arg!(<PATH> "file path")),
                ),
        )
        .subcommand(
            Command::new("json")
                .about("JSON utilities")
                .subcommand(
                    Command::new("encode")
                        .about("JSON encode demo"),
                ),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
        .get_matches();

    match matches.subcommand() {
        Some(("gen", sub_match)) => {
            println!("hello {:?}", sub_match.get_many::<String>("gen").unwrap());
            println!("Hello, world!");
            files::file_control::get_all_lines();
            json_opera();
        }
        Some(("string", string_matches)) => {
            match string_matches.subcommand() {
                Some(("reverse", m)) => {
                    let text = m.get_one::<String>("TEXT").expect("required");
                    println!("{}", string_util::reverse(text));
                }
                Some(("count", m)) => {
                    let text = m.get_one::<String>("TEXT").expect("required");
                    println!("Word count: {}", string_util::word_count(text));
                }
                _ => unreachable!(),
            }
        }
        Some(("path", path_matches)) => {
            match path_matches.subcommand() {
                Some(("ext", m)) => {
                    let p = m.get_one::<String>("PATH").expect("required");
                    match path_util::get_extension(p) {
                        Some(ext) => println!("Extension: {}", ext),
                        None => println!("No extension found"),
                    }
                }
                Some(("parent", m)) => {
                    let p = m.get_one::<String>("PATH").expect("required");
                    match path_util::get_parent(p) {
                        Some(parent) => println!("Parent: {}", parent),
                        None => println!("No parent (root or empty path)"),
                    }
                }
                _ => unreachable!(),
            }
        }
        Some(("json", _)) => {
            json_opera();
        }
        _ => todo!(),
    }
}

fn json_opera() {
    json_util::json_decode();
}
