use std::process::ExitCode;

use clap::CommandFactory;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use guess::core_ops;

/// Guess CLI - A multi-purpose command tool
#[derive(Debug, Parser)]
#[command(name = "guess", version, author, about = "A multi-purpose CLI tool", long_about = None)]
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
        #[arg(long, default_value_t = 1)]
        min: i32,
        /// Maximum value
        #[arg(long, default_value_t = 100)]
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
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Hash { text } => Ok(format!("MD5({text:?}) = {}", core_ops::hash_md5(&text))),
        Commands::Fetch { url } => core_ops::fetch(&url).await,
        Commands::Random { min, max } => Ok(format!(
            "Random number ({min}..={max}): {}",
            core_ops::random_in_range(min, max)
        )),
        Commands::Read { path } => core_ops::read_text(&path),
        Commands::Info => Ok(core_ops::info()),
        Commands::Core => {
            guess::core::hyper::main_core();
            Ok(String::new())
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(shell, &mut cmd, name, &mut out);
            Ok(String::from_utf8_lossy(&out).into_owned())
        }
    };

    match result {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}
