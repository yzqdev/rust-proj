use clap::{Arg, Command, arg};
use mini::{files, json_util};

fn main() {
    let matches = Command::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(Arg::new("in_file"))
        .subcommand(
            Command::new("gen")
                .about("Clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
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
            // read_lines();
            json_opera();
        }
        _ => todo!(),
    }
}
fn json_opera() {
    json_util::json_decode();
}
