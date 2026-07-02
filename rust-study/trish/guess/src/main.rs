use clap::{Parser, Subcommand};
use md5::Digest;
use guess::core;
use guess::util;

/// Guess CLI - A multi-purpose command tool
#[derive(Debug, Parser)]
#[command(name = "guess")]
#[command(about = "A multi-purpose CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Calculate MD5 hash of a string
    Hash {
        /// The string to hash
        text: String,
    },
    /// Make an HTTP request
    Fetch {
        /// URL to fetch
        url: String,
    },
    /// Generate a random number
    Random {
        /// Minimum value
        #[arg(short, long, default_value_t = 1)]
        min: i32,
        /// Maximum value
        #[arg(short, long, default_value_t = 100)]
        max: i32,
    },
    /// Read and display a text file
    Read {
        /// File path
        path: String,
    },
    /// Show system information
    Info,
    /// Run core demo
    Core,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hash { text } => {
            let digest = md5::Md5::digest(text.as_bytes());
            println!("MD5({:?}) = {:x}", text, digest);
        }
        Commands::Fetch { url } => {
            println!("Fetching {} ...", url);
            // note: reqwest needs tokio runtime
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match reqwest::get(&url).await {
                    Ok(resp) => {
                        println!("Status: {}", resp.status());
                        if let Ok(body) = resp.text().await {
                            println!("Body (first 500 chars):");
                            let preview: String = body.chars().take(500).collect();
                            println!("{}", preview);
                        }
                    }
                    Err(e) => eprintln!("Request failed: {}", e),
                }
            });
        }
        Commands::Random { min, max } => {
            use rand::Rng;
            let num = rand::thread_rng().gen_range(min..=max);
            println!("Random number ({}..={}): {}", min, max, num);
        }
        Commands::Read { path } => {
            match std::fs::read_to_string(&path) {
                Ok(content) => println!("{}", content),
                Err(e) => eprintln!("Failed to read {}: {}", path, e),
            }
        }
        Commands::Info => {
            println!("=== System Info ===");
            println!("OS: {}", std::env::consts::OS);
            println!("Arch: {}", std::env::consts::ARCH);
            println!("Current dir: {:?}", std::env::current_dir().unwrap_or_default());
        }
        Commands::Core => {
            core::hyper::main_core();
        }
    }
}
