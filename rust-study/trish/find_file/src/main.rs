use std::process::ExitCode;

use clap::{Command, arg};
use clap_complete::Shell;
use find_file::{find_by_pattern, find_empty, find_large, find_recent, search};

fn cli() -> Command {
    Command::new("find_file")
        .version(clap::crate_version!())
        .author("yzqdev")
        .about("Find files by pattern, size, recency or emptiness")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("hostname").about("show hostname part of this machine"))
        .subcommand(
            Command::new("png")
                .about("find png files")
                .arg(arg!(<PATH> "find png path")),
        )
        .subcommand(
            Command::new("txt")
                .about("find text files")
                .arg(arg!(<PATH> "find txt path")),
        )
        .subcommand(
            Command::new("large")
                .about("find files larger than specified size (bytes)")
                .arg(arg!(<PATH> "search path"))
                .arg(arg!(<SIZE> "minimum size in bytes")),
        )
        .subcommand(
            Command::new("recent")
                .about("find recently modified files (within N days)")
                .arg(arg!(<PATH> "search path"))
                .arg(arg!(<DAYS> "number of days")),
        )
        .subcommand(
            Command::new("empty")
                .about("find empty files and directories")
                .arg(arg!(<PATH> "search path")),
        )
        .subcommand(
            Command::new("query")
                .about("search for lines containing a query in a text file")
                .arg(arg!(<QUERY> "text to search for"))
                .arg(arg!(<FILE> "file to search in")),
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
}

fn required(matches: &clap::ArgMatches, id: &str) -> Result<String, String> {
    matches
        .get_one::<String>(id)
        .cloned()
        .ok_or_else(|| format!("missing required argument `{id}`"))
}

fn parse_number<T: std::str::FromStr>(value: &str, what: &str) -> Result<T, String> {
    value
        .parse::<T>()
        .map_err(|_| format!("{what} must be a number, got `{value}`"))
}

fn dispatch(matches: clap::ArgMatches) -> Result<Vec<String>, String> {
    match matches.subcommand() {
        Some(("hostname", _)) => {
            let name = std::env::var("COMPUTERNAME")
                .or_else(|_| std::env::var("HOSTNAME"))
                .unwrap_or_else(|_| "unknown".to_string());
            Ok(vec![name])
        }
        Some(("png", m)) => {
            let path = required(m, "PATH")?;
            Ok(find_by_pattern(&path, "png")
                .into_iter()
                .map(|p| p.display().to_string())
                .collect())
        }
        Some(("txt", m)) => {
            let path = required(m, "PATH")?;
            Ok(find_by_pattern(&path, "txt")
                .into_iter()
                .map(|p| p.display().to_string())
                .collect())
        }
        Some(("large", m)) => {
            let path = required(m, "PATH")?;
            let size = parse_number(&required(m, "SIZE")?, "size")?;
            Ok(find_large(&path, size)
                .into_iter()
                .map(|(p, s)| format!("{} ({} bytes)", p.display(), s))
                .collect())
        }
        Some(("recent", m)) => {
            let path = required(m, "PATH")?;
            let days = parse_number(&required(m, "DAYS")?, "days")?;
            Ok(find_recent(&path, days)
                .into_iter()
                .map(|p| p.display().to_string())
                .collect())
        }
        Some(("empty", m)) => {
            let path = required(m, "PATH")?;
            Ok(find_empty(&path)
                .into_iter()
                .map(|(p, kind)| match kind {
                    "dir" => format!("[dir]  {}", p.display()),
                    _ => format!("[file] {}", p.display()),
                })
                .collect())
        }
        Some(("query", m)) => {
            let query = required(m, "QUERY")?;
            let file = required(m, "FILE")?;
            let contents =
                std::fs::read_to_string(&file).map_err(|e| format!("cannot read `{file}`: {e}"))?;
            Ok(search(&query, &contents)
                .into_iter()
                .map(str::to_string)
                .collect())
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
