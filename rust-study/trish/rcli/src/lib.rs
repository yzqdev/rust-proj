pub mod util;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Encode text to base64
    Encode {
        /// Text to encode
        text: String,
    },
    /// Decode base64 text
    Decode {
        /// Base64 text to decode
        text: String,
    },
    /// Generate a random password
    Genpwd {
        /// Length of the password
        #[arg(short, long, default_value_t = 16)]
        length: u8,
        /// Include special characters
        #[arg(short, long)]
        special: bool,
    },
    /// Count lines, words, and chars in text or file
    Count {
        /// Text or file path
        input: String,
        /// Treat input as file path
        #[arg(short, long)]
        file: bool,
    },
    /// Calculate time benchmark
    Bench,
}
