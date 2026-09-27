use std::process::ExitCode;

use clap::{Command, arg, command};
use clap_complete::Shell;
use mini::{files, json_util, path_util, string_util};

fn cli() -> Command {
    command!("mini")
        .version(clap::crate_version!())
        .about("Mini utility tool - file, json, string and path operations")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("gen")
                .about("Run generation demo (file ops + json)")
                .arg(arg!([NAME] "Optional demo name")),
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
                )
                .subcommand(
                    Command::new("snake")
                        .about("Convert to snake_case")
                        .arg(arg!(<TEXT> "text to convert")),
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
                .subcommand(Command::new("encode").about("JSON encode demo")),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    arg!(--shell <SHELL> "Shell to generate completions for")
                        .value_parser(clap::value_parser!(Shell))
                        .required(true),
                ),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
}

fn dispatch(matches: clap::ArgMatches) {
    match matches.subcommand() {
        Some(("gen", _sub_match)) => {
            files::file_control::get_all_lines();
            json_util::json_decode();
        }
        Some(("string", string_matches)) => match string_matches.subcommand() {
            Some(("reverse", m)) => {
                let text = m.get_one::<String>("TEXT").expect("required by clap");
                println!("{}", string_util::reverse(text));
            }
            Some(("count", m)) => {
                let text = m.get_one::<String>("TEXT").expect("required by clap");
                println!("Word count: {}", string_util::word_count(text));
            }
            Some(("snake", m)) => {
                let text = m.get_one::<String>("TEXT").expect("required by clap");
                println!("{}", string_util::to_snake_case(text));
            }
            _ => unreachable!("parser should ensure only valid subcommand names are used"),
        },
        Some(("path", path_matches)) => match path_matches.subcommand() {
            Some(("ext", m)) => {
                let p = m.get_one::<String>("PATH").expect("required by clap");
                match path_util::get_extension(p) {
                    Some(ext) => println!("Extension: {ext}"),
                    None => println!("No extension found"),
                }
            }
            Some(("parent", m)) => {
                let p = m.get_one::<String>("PATH").expect("required by clap");
                match path_util::get_parent(p) {
                    Some(parent) => println!("Parent: {parent}"),
                    None => println!("No parent (root or empty path)"),
                }
            }
            _ => unreachable!("parser should ensure only valid subcommand names are used"),
        },
        Some(("json", _)) => {
            json_util::json_decode();
        }
        Some(("completions", m)) => {
            let shell = m.get_one::<Shell>("shell").expect("required by clap");
            let mut cmd = cli();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            print!("{}", String::from_utf8_lossy(&out));
        }
        _ => unreachable!("subcommand_required(true) makes this unreachable"),
    }
}

fn main() -> ExitCode {
    dispatch(cli().get_matches());
    ExitCode::SUCCESS
}
