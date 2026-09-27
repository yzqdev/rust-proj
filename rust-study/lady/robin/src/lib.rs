//! robin - File utility CLI.
//!
//! Compute file hashes, inspect file info, display directory trees.

use clap::{CommandFactory, Parser};
use clap_complete::Shell;

pub mod cmd;

use crate::cmd::file_cmd;

/// robin - File utility CLI
///
/// Compute file hashes, inspect file info, display directory trees.
#[derive(Debug, Parser)]
#[command(
    version,
    author,
    about = "File utility CLI",
    long_about = "robin - File utility CLI\n\nCompute file hashes, inspect file info, display directory trees."
)]
pub struct Cli {
    #[command(subcommand)]
    pub sub: SubCmd,
}

#[derive(Debug, Parser)]
pub enum SubCmd {
    /// Demo command: echoes the given number
    Add {
        #[arg(short, long)]
        num: u16,
    },
    /// Compute MD5 hash of a file
    #[command(name = "md5")]
    Md5 {
        /// Path to the file
        file_name: String,
    },
    /// Show image file information
    #[command(name = "img")]
    Image {
        /// Path to the image file
        file_name: String,
    },
    /// Show detailed file information
    #[command(name = "info")]
    Info {
        /// Path to the file
        file_name: String,
    },
    /// Display directory tree
    #[command(name = "tree")]
    Tree {
        /// Directory to display
        dir_name: String,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
}

/// Errors reported by the CLI layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The given path does not exist.
    #[error("file not found: {0}")]
    NotFound(String),

    /// The given path is not a directory.
    #[error("`{0}` is not a valid directory")]
    NotADirectory(String),

    /// A filesystem operation failed.
    #[error("failed to access `{path}`: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<hash_utils::Error> for Error {
    fn from(err: hash_utils::Error) -> Self {
        match err {
            hash_utils::Error::NotFound(path) => Error::NotFound(path),
            hash_utils::Error::Io { path, source } => Error::Io { path, source },
        }
    }
}

/// Execute a parsed command and return the text to print on stdout.
pub fn run(cli: &Cli) -> Result<String> {
    match &cli.sub {
        SubCmd::Add { num } => Ok(format!("add num: {num}")),
        SubCmd::Md5 { file_name } => file_cmd::calc_md5(file_name),
        SubCmd::Image { file_name } => file_cmd::image_info(file_name),
        SubCmd::Info { file_name } => file_cmd::file_info(file_name),
        SubCmd::Tree { dir_name } => file_cmd::dir_tree(dir_name),
        SubCmd::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            Ok(String::from_utf8_lossy(&out).into_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_demo_echoes_number() {
        let cli = Cli {
            sub: SubCmd::Add { num: 42 },
        };
        assert_eq!(run(&cli).unwrap(), "add num: 42");
    }
}
