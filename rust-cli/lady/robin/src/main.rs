use clap::{arg, command, Parser, Subcommand};
use cmd::file_cmd::{calc_md5, FileArgs, FileCmd};
use util::gen_fsmd5;
mod cmd;
pub mod util;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub: Option<SubCmd>,
}
#[derive(Subcommand, Debug)]
enum SubCmd {
    /// Add a number
    Add {
        #[arg(short, long)]
        num: u16,
    },

    File(FileArgs),
}
fn main() {
    let cli = Cli::parse();
    match cli.sub {
        Some(SubCmd::Add { num }) => println!("test add num: {:?}", num),
        Some(SubCmd::File(file_args)) => {
            let file_cmd = file_args.command.unwrap();
            match file_cmd {
                FileCmd::Image => {}
                FileCmd::Md5 { file_name } => {
                    calc_md5(file_name.as_str());
                }
            }
        }
        None => print!(""),
    }
}
