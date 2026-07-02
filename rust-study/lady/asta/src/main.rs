use std::{fs, io::Read};

use clap::{Parser, Subcommand};
use digest::Digest;

/// asta - Hash calculator (MD5, SHA256)
///
/// Compute message digests for strings and files.
#[derive(Parser)]
#[command(name = "asta", version = "1.0", author = "yzqdev", about = "Hash calculator CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    /// Compute MD5 hash of a file
    File {
        /// Path to the file
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Md5 { text } => {
            let hash = md5::Md5::digest(text.as_bytes());
            println!("MD5(\"{}\") = {:x}", text, hash);
        }
        Commands::Sha256 { text } => {
            let hash = sha2::Sha256::digest(text.as_bytes());
            println!("SHA256(\"{}\") = {:x}", text, hash);
        }
        Commands::File { path } => {
            let mut file = fs::File::open(&path).unwrap_or_else(|e| {
                eprintln!("Error: cannot open '{}': {}", path, e);
                std::process::exit(1);
            });
            let mut hasher = md5::Md5::new();
            let mut buffer = [0u8; 8192];
            loop {
                let n = file.read(&mut buffer).unwrap_or_else(|e| {
                    eprintln!("Error: read failed: {}", e);
                    std::process::exit(1);
                });
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }
            let hash = hasher.finalize();
            println!("MD5({}) = {:x}", path, hash);
        }
    }
}
