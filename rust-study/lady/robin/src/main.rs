use clap::Parser;

pub mod cmd;
pub mod util;

/// robin - File utility CLI
///
/// Compute file hashes, inspect file info, display directory trees.
#[derive(Parser)]
#[command(version, author, about = "File utility CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    sub: SubCmd,
}

#[derive(Parser, Debug)]
enum SubCmd {
    /// Add two numbers (demo command)
    Add {
        #[arg(short, long)]
        num: u16,
    },
    /// Compute MD5 hash of a file
    #[command(name = "md5")]
    Md5 {
        #[arg(help = "Path to the file")]
        file_name: String,
    },
    /// Show image file information
    #[command(name = "img")]
    Image {
        #[arg(help = "Path to the image file")]
        file_name: String,
    },
    /// Show detailed file information
    #[command(name = "info")]
    Info {
        #[arg(help = "Path to the file")]
        file_name: String,
    },
    /// Display directory tree
    #[command(name = "tree")]
    Tree {
        #[arg(help = "Directory to display")]
        dir_name: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.sub {
        SubCmd::Add { num } => println!("add num: {:?}", num),
        SubCmd::Md5 { file_name } => {
            cmd::file_cmd::calc_md5(&file_name);
        }
        SubCmd::Image { file_name } => {
            cmd::file_cmd::image_info(&file_name);
        }
        SubCmd::Info { file_name } => {
            cmd::file_cmd::file_info(&file_name);
        }
        SubCmd::Tree { dir_name } => {
            cmd::file_cmd::dir_tree(&dir_name);
        }
    }
}
