//! asta - Hash calculator (MD5, SHA256).
//!
//! Library crate so the command definitions and execution logic can be
//! unit tested without spawning the binary.

use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

/// asta - Hash calculator (MD5, SHA256)
///
/// Compute message digests for strings and files.
#[derive(Debug, Parser)]
#[command(
    name = "asta",
    version,
    author,
    about = "Hash calculator CLI",
    long_about = "asta - Hash calculator (MD5, SHA256)\n\nCompute message digests for strings and files."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported file hash algorithms.
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum FileAlgorithm {
    /// MD5 digest
    Md5,
    /// SHA256 digest
    Sha256,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Compute MD5 hash of a string
    Md5 {
        /// The string to hash
        text: String,
    },
    /// Compute SHA256 hash of a string
    Sha256 {
        /// The string to hash
        text: String,
    },
    /// Compute the hash of a file
    File {
        /// Path to the file
        path: PathBuf,
        /// Hash algorithm to use
        #[arg(short, long, value_enum, default_value_t = FileAlgorithm::Md5)]
        algorithm: FileAlgorithm,
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
    #[error(transparent)]
    Hash(#[from] hash_utils::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Execute a parsed command and return the text to print on stdout.
/// Kept separate from I/O so tests can drive it.
pub fn run(cli: &Cli) -> Result<String> {
    match &cli.command {
        Commands::Md5 { text } => Ok(format!(
            "MD5({text:?}) = {}",
            hash_utils::md5_hex(text.as_bytes())
        )),
        Commands::Sha256 { text } => Ok(format!(
            "SHA256({text:?}) = {}",
            hash_utils::sha256_hex(text.as_bytes())
        )),
        Commands::File { path, algorithm } => {
            let algo = match algorithm {
                FileAlgorithm::Md5 => hash_utils::Algorithm::Md5,
                FileAlgorithm::Sha256 => hash_utils::Algorithm::Sha256,
            };
            Ok(format!(
                "{}({}) = {}",
                label(*algorithm),
                path.display(),
                hash_utils::file_hash(path, algo)?
            ))
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            Ok(String::from_utf8_lossy(&out).into_owned())
        }
    }
}

fn label(algorithm: FileAlgorithm) -> &'static str {
    match algorithm {
        FileAlgorithm::Md5 => "MD5",
        FileAlgorithm::Sha256 => "SHA256",
    }
}

impl Cli {
    /// Parse process arguments.
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
