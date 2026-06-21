use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use clap::Args;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

#[derive(Args)]
pub struct DedupArgs {
    /// Directory to scan for duplicates
    #[arg(required = true)]
    pub path: String,

    /// Dry run (show duplicates without deleting)
    #[arg(long)]
    pub dry_run: bool,

    /// Delete duplicate files (keeps the first occurrence)
    #[arg(long)]
    pub delete: bool,

    /// Minimum file size to consider (skip tiny files)
    #[arg(long, default_value_t = 1)]
    pub min_size: u64,

    /// Only consider files matching this extension
    #[arg(long)]
    pub ext: Option<String>,
}

pub fn execute(args: &DedupArgs) {
    let root = Path::new(&args.path);
    if !root.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.path);
        return;
    }

    eprintln!("Scanning for duplicates...");

    let mut size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if let Some(ref ext) = args.ext {
            if !path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case(ext))
                .unwrap_or(false)
            {
                continue;
            }
        }

        if let Ok(meta) = fs::metadata(path) {
            if meta.len() >= args.min_size {
                size_map
                    .entry(meta.len())
                    .or_default()
                    .push(path.to_path_buf());
            }
        }
    }

    let candidates: Vec<&Vec<PathBuf>> = size_map.values().filter(|v| v.len() > 1).collect();

    if candidates.is_empty() {
        println!("No duplicate files found");
        return;
    }

    let mut total_groups = 0;
    let mut total_duplicates = 0;
    let mut total_wasted = 0u64;

    for group in &candidates {
        let mut hashes: HashMap<String, Vec<&PathBuf>> = HashMap::new();

        for path in *group {
            if let Ok(hash) = compute_quick_hash(path) {
                hashes.entry(hash).or_default().push(path);
            }
        }

        for (hash, files) in &hashes {
            if files.len() < 2 {
                continue;
            }

            total_groups += 1;
            total_duplicates += files.len() - 1;

            let size = fs::metadata(files[0]).map(|m| m.len()).unwrap_or(0);
            total_wasted += size * (files.len() as u64 - 1);

            let short_hash = &hash[..std::cmp::min(16, hash.len())];
            println!("\n  Group #{total_groups} (hash: {short_hash}..., size: {}):", format_size(size));
            for (i, file) in files.iter().enumerate() {
                let marker = if i == 0 { "  [KEEP] " } else { "  [DUP]  " };
                println!("{marker}{}", file.display());

                if !args.dry_run && args.delete && i > 0 {
                    match fs::remove_file(file) {
                        Ok(()) => println!("           -> Deleted"),
                        Err(e) => eprintln!("           -> Failed to delete: {e}"),
                    }
                }
            }
        }
    }

    println!("\n  Summary:");
    println!("    Duplicate groups: {total_groups}");
    println!("    Duplicate files:  {total_duplicates}");
    println!("    Wasted space:     {}", format_size(total_wasted));

    if args.dry_run {
        println!("\n  Dry run - no files were deleted");
    } else if args.delete {
        println!("\n  Duplicate files have been deleted");
    } else {
        println!("\n  Use --delete to remove duplicates, or --dry-run to preview");
    }
}

fn compute_quick_hash(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Ok(format!("{:x}", hasher.finalize()))
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
