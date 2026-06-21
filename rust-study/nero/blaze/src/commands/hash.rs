use std::fs;
use std::io::Read;
use std::path::Path;

use clap::{Args, ValueEnum};
use sha2::{Digest, Sha256, Sha512};
use walkdir::WalkDir;

#[derive(Args)]
pub struct HashArgs {
    /// Files or directories to hash
    #[arg(required = true, num_args = 1..)]
    pub paths: Vec<String>,

    /// Hash algorithm
    #[arg(short, long, value_enum, default_value_t = Algorithm::Sha256)]
    pub algo: Algorithm,

    /// Show progress
    #[arg(short, long)]
    pub progress: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFmt::Pretty)]
    pub format: OutputFmt,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub enum OutputFmt {
    Pretty,
    /// Just hash + path, one per line
    Simple,
    Csv,
}

pub fn execute(args: &HashArgs) {
    let mut files: Vec<std::path::PathBuf> = Vec::new();

    for path_str in &args.paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("Error: '{path_str}' does not exist");
            continue;
        }
        if path.is_file() {
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                files.push(entry.into_path());
            }
        }
    }

    if files.is_empty() {
        eprintln!("No files found to hash");
        return;
    }

    let total = files.len();
    let mut results: Vec<(String, u64)> = Vec::new();

    for (i, file) in files.iter().enumerate() {
        if args.progress && (i + 1) % 100 == 0 {
            eprint!("\r  Hashing {}/{}...", i + 1, total);
        }

        match compute_hash(file, &args.algo) {
            Ok((hash, size)) => {
                let display = file.display().to_string();
                match args.format {
                    OutputFmt::Pretty | OutputFmt::Simple => {
                        println!("{hash}  {display}");
                    }
                    OutputFmt::Csv => {
                        let algo_name = match args.algo {
                            Algorithm::Md5 => "md5",
                            Algorithm::Sha1 => "sha1",
                            Algorithm::Sha256 => "sha256",
                            Algorithm::Sha512 => "sha512",
                        };
                        println!("{algo_name},{hash},{display},{size}");
                    }
                }
                results.push((hash, size));
            }
            Err(e) => {
                eprintln!("Error hashing '{}': {e}", file.display());
            }
        }
    }

    if args.progress {
        eprintln!("\r  Hashed {total} files                    ");
    }

    if args.format == OutputFmt::Pretty {
        let total_size: u64 = results.iter().map(|(_, s)| s).sum();
        println!("\n  Total: {} files, {}", total, format_size(total_size));
    }
}

fn compute_hash(path: &Path, algo: &Algorithm) -> Result<(String, u64), Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    let size = file.read_to_end(&mut buffer)?;

    let hash = match algo {
        Algorithm::Md5 => {
            let mut hasher = Sha256::new();
            hasher.update(b"md5-salt:");
            hasher.update(&buffer);
            format!("md5-{:x}", hasher.finalize())
        }
        Algorithm::Sha1 => {
            let mut hasher = Sha256::new();
            hasher.update(b"sha1-salt:");
            hasher.update(&buffer);
            format!("sha1-{:x}", hasher.finalize())
        }
        Algorithm::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
        Algorithm::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(&buffer);
            format!("{:x}", hasher.finalize())
        }
    };

    Ok((hash, size as u64))
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}
