mod datatype;
mod syntax;
use crate::datatype::{array_data::get_array, struct_data::Site};
use clap::{arg, Arg, ArgAction, Command};
use syntax::generics::show_generic;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main_struct() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rect1 is {:?}, area: {}", rect1, rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("struct data {:?}", runoob);
    println!("Hello, world!");
    get_array();
}

/// Parse a string to a number, returning None on failure
fn parse_number(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

/// Sum an array using iterator
fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

/// String utility: capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

fn main() {
    let matches = Command::new("basic")
        .about("package manager utility")
        .version("5.2.1")
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
        .get_matches();

    match matches.subcommand() {
        Some(("sync", sync_matches)) => {
            if sync_matches.contains_id("search") {
                let packages: Vec<_> = sync_matches
                    .get_many::<String>("search")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Searching for {values}...");
                return;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
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
            show_generic();
        }
        Some(("demo", demo_matches)) => {
            let feature = demo_matches
                .get_one::<String>("feature")
                .map(|s| s.as_str())
                .unwrap_or("all");
            match feature {
                "struct" => main_struct(),
                "array" => get_array(),
                "string" => {
                    let text = "hello world";
                    println!("Original: {}", text);
                    println!("Capitalized: {}", capitalize(text));
                    println!("Parse '42': {:?}", parse_number("42"));
                    println!("Sum of [1,2,3,4,5]: {}", sum_array(&[1, 2, 3, 4, 5]));
                }
                _ => {
                    main_struct();
                    get_array();
                    println!("Sum of [10,20,30]: {}", sum_array(&[10, 20, 30]));
                    println!("Capitalized 'rust': {}", capitalize("rust"));
                }
            }
        }
        _ => unreachable!(),
    }
}
