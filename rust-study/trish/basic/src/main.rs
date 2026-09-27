use std::process::ExitCode;

use basic::{string_demo, struct_demo};
use clap::{Arg, ArgAction, Command, arg};
use clap_complete::Shell;

fn cli() -> Command {
    Command::new("basic")
        .version(clap::crate_version!())
        .author("yzqdev")
        .about("package manager utility (clap builder API demo)")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the package database.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("search locally installed packages for matching strings")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .short('i')
                        .conflicts_with("search")
                        .help("view package information")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("sync")
                .short_flag('S')
                .long_flag("sync")
                .about("Synchronize packages.")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("search remote repositories for matching strings"),
                )
                .arg(
                    Arg::new("info")
                        .long("info")
                        .conflicts_with("search")
                        .short('i')
                        .action(ArgAction::SetTrue)
                        .help("view package information"),
                )
                .arg(arg!(<name> "package name"))
                .arg(
                    Arg::new("package")
                        .help("packages")
                        .required_unless_present("search")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(Command::new("generic"))
        .subcommand(
            Command::new("demo")
                .about("Show various Rust feature demos")
                .arg(
                    Arg::new("feature")
                        .short('f')
                        .long("feature")
                        .help("Feature to demo: struct, array, string, all")
                        .default_value("all"),
                ),
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

fn main() -> ExitCode {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if let Some(packages) = sync_matches.get_many::<String>("search") {
                let values = packages.cloned().collect::<Vec<_>>().join(", ");
                println!("Searching for {values}...");
                return ExitCode::SUCCESS;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();
            let values = packages.join(", ");

            if sync_matches.get_flag("info") {
                println!("Retrieving info for {values}...");
            } else {
                println!("Installing {values}...");
            }
        }
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("info") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {comma_sep}...");
            } else if let Some(queries) = query_matches.get_many::<String>("search") {
                let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {comma_sep}...");
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        Some(("generic", _)) => {
            basic::syntax::generics::show_generic();
        }
        Some(("demo", demo_matches)) => {
            let feature = demo_matches
                .get_one::<String>("feature")
                .map(|s| s.as_str())
                .unwrap_or("all");
            match feature {
                "struct" => struct_demo(),
                "array" => basic::datatype::array_data::get_array(),
                "string" => string_demo(),
                _ => {
                    struct_demo();
                    string_demo();
                    println!("Sum of [10,20,30]: {}", basic::sum_array(&[10, 20, 30]));
                    println!("Capitalized 'rust': {}", basic::capitalize("rust"));
                }
            }
        }
        Some(("completions", m)) => {
            let shell = m.get_one::<Shell>("shell").expect("required by clap");
            let mut cmd = cli();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            print!("{}", String::from_utf8_lossy(&out));
        }
        // `subcommand_required(true)` makes this unreachable.
        _ => return ExitCode::FAILURE,
    }
    ExitCode::SUCCESS
}
