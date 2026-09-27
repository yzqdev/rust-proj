use std::process::ExitCode;

use clap::{Command, arg};
use clap_complete::Shell;
use study::io_use::simple_fs::{add, compute_md5, file_size, list_dir};

fn cli() -> Command {
    Command::new("study")
        .version(clap::crate_version!())
        .author("yzqdev")
        .about("Study tool - file operations, string utils, and more")
        .subcommand_required(true)
        .arg_required_else_help(true)
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

fn required(matches: &clap::ArgMatches, id: &str) -> Result<String, String> {
    matches
        .get_one::<String>(id)
        .cloned()
        .ok_or_else(|| format!("missing required argument `{id}`"))
}

fn dispatch(matches: clap::ArgMatches) -> Result<Vec<String>, String> {
    match matches.subcommand() {
        Some(("clone", sub_matches)) => {
            let remote = required(sub_matches, "REMOTE")?;
            Ok(vec![format!("Cloning {remote}")])
        }
        Some(("md5", sub_matches)) => {
            let file = required(sub_matches, "FILE")?;
            compute_md5(&file)
                .map(|hash| vec![format!("MD5({file}) = {hash}")])
                .map_err(|e| format!("cannot read `{file}`: {e}"))
        }
        Some(("size", sub_matches)) => {
            let file = required(sub_matches, "FILE")?;
            file_size(&file)
                .map(|line| vec![line])
                .map_err(|e| format!("cannot read `{file}`: {e}"))
        }
        Some(("ls", sub_matches)) => {
            let dir = required(sub_matches, "DIR")?;
            list_dir(&dir)
                .map(|line| vec![line])
                .map_err(|e| format!("cannot read directory `{dir}`: {e}"))
        }
        Some(("add", sub_matches)) => {
            let a: i32 = required(sub_matches, "A")?
                .parse()
                .map_err(|_| "A must be a number".to_string())?;
            let b: i32 = required(sub_matches, "B")?
                .parse()
                .map_err(|_| "B must be a number".to_string())?;
            Ok(vec![format!("{a} + {b} = {}", add(a, b))])
        }
        Some(("completions", m)) => {
            let shell = m.get_one::<Shell>("shell").expect("required by clap");
            let mut cmd = cli();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            Ok(vec![String::from_utf8_lossy(&out).into_owned()])
        }
        // `subcommand_required(true)` makes this unreachable.
        _ => Err("no subcommand given".into()),
    }
}

fn main() -> ExitCode {
    match dispatch(cli().get_matches()) {
        Ok(lines) => {
            for line in lines {
                println!("{line}");
            }
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("Error: {message}");
            ExitCode::FAILURE
        }
    }
}
