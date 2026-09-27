// 教学示例。
#![allow(unused_variables, unused_assignments)]
#![allow(clippy::all)]

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// Teaching example (not wired to the CLI): a standalone clap derive parser.
pub fn simple_cmd() {
    let args = Args::parse();

    for i in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
