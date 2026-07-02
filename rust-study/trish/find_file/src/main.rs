use clap::{Command, arg};
use find_file::{Config, main_fun};
use glob::glob;
use std::fs;

fn main() {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .arg_required_else_help(true)
        .subcommand(Command::new("hostname").about("show hostname part of FQDN"))
        .subcommand(Command::new("gen"))
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
            let pattern = format!("{}/**/*.png", path);
            for entry in glob(&pattern).unwrap().flatten() {
                println!("{}", entry.display());
            }
        }
        Some(("txt", txt_match)) => {
            let path = txt_match.get_one::<String>("PATH").expect("parser error");
            let pattern = format!("{}/**/*.txt", path);
            for entry in glob(&pattern).unwrap().flatten() {
                println!("{}", entry.display());
            }
        }
        Some(("large", large_match)) => {
            let path = large_match.get_one::<String>("PATH").expect("parser error");
            let size: u64 = large_match
                .get_one::<String>("SIZE")
                .expect("parser error")
                .parse()
                .expect("size must be a number");
            let pattern = format!("{}/**/*", path);
            for entry in glob(&pattern).unwrap().flatten() {
                if let Ok(meta) = fs::metadata(&entry) {
                    if meta.len() > size {
                        println!("{} ({} bytes)", entry.display(), meta.len());
                    }
                }
            }
        }
        Some(("recent", recent_match)) => {
            let path = recent_match.get_one::<String>("PATH").expect("parser error");
            let days: u64 = recent_match
                .get_one::<String>("DAYS")
                .expect("parser error")
                .parse()
                .expect("days must be a number");
            let now = std::time::SystemTime::now();
            let duration = std::time::Duration::from_secs(days * 24 * 3600);
            let threshold = now - duration;
            let pattern = format!("{}/**/*", path);
            for entry in glob(&pattern).unwrap().flatten() {
                if let Ok(meta) = fs::metadata(&entry) {
                    if let Ok(modified) = meta.modified() {
                        if modified > threshold {
                            println!("{}", entry.display());
                        }
                    }
                }
            }
        }
        Some(("empty", empty_match)) => {
            let path = empty_match.get_one::<String>("PATH").expect("parser error");
            let pattern = format!("{}/**/*", path);
            for entry in glob(&pattern).unwrap().flatten() {
                if entry.is_dir() {
                    if entry.read_dir().map(|mut d| d.next().is_none()).unwrap_or(false) {
                        println!("[dir]  {}", entry.display());
                    }
                } else if let Ok(meta) = fs::metadata(&entry) {
                    if meta.len() == 0 {
                        println!("[file] {}", entry.display());
                    }
                }
            }
        }
        Some(("gen", _)) => {
            println!("gen")
        }
        _ => unreachable!("parser should ensure only valid subcommand names are used"),
    }
}
