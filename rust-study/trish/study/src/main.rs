mod io_use;

use std::time::Instant;

use clap::{Arg, Command, arg};

use crate::io_use::simple_fs::{add, get_file_md5};
fn main() {
    let m = Command::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(Arg::new("in_file"))
        .subcommand(
            Command::new("clone")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
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
        _ => unreachable!(),
    }
}
