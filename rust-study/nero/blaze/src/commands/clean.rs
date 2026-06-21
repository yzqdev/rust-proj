use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use clap::Args;
use walkdir::WalkDir;

#[derive(Args)]
pub struct CleanArgs {
    /// Directory to clean
    #[arg(required = true)]
    pub path: String,

    /// Remove files older than this (e.g., 7d, 30d, 1y)
    #[arg(short, long)]
    pub min_age: Option<String>,

    /// Remove files smaller than this (e.g., 0b, 1kb)
    #[arg(long)]
    pub max_size: Option<String>,

    /// Clean specific file types
    #[arg(short, long)]
    pub ext: Option<String>,

    /// Remove empty directories
    #[arg(long)]
    pub empty_dirs: bool,

    /// Remove common temp/cache files (*.tmp, *.log, __pycache__, .DS_Store, etc.)
    #[arg(long)]
    pub temp: bool,

    /// Dry run (show what would be removed)
    #[arg(long)]
    pub dry_run: bool,

    /// Recurse into subdirectories
    #[arg(short, long)]
    pub recursive: bool,
}

pub fn execute(args: &CleanArgs) {
    let root = Path::new(&args.path);
    if !root.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.path);
        return;
    }

    let min_age_duration = args.min_age.as_deref().map(parse_duration);

    let max_size_bytes = args.max_size.as_deref().and_then(|s| parse_size(s).ok());

    let mut files_to_remove: Vec<PathBuf> = Vec::new();
    let mut dirs_to_remove: Vec<PathBuf> = Vec::new();

    let walker = WalkDir::new(root)
        .max_depth(if args.recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        let path = entry.path();
        let meta = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.is_file() {
            let mut should_remove = false;

            // Temp files filter
            if args.temp && is_temp_file(path) {
                should_remove = true;
            }

            // Extension filter
            if let Some(ref ext) = args.ext {
                if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case(ext))
                    .unwrap_or(false)
                {
                    should_remove = true;
                }
            }

            // Age filter
            if let Some(duration) = min_age_duration {
                if let Ok(modified) = meta.modified() {
                    if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                        if elapsed >= duration {
                            should_remove = true;
                        }
                    }
                }
            }

            // Size filter
            if let Some(max) = max_size_bytes {
                if meta.len() <= max {
                    should_remove = true;
                }
            }

            if should_remove {
                files_to_remove.push(path.to_path_buf());
            }
        } else if meta.is_dir() && args.empty_dirs && path != root {
            if let Ok(entries) = fs::read_dir(path) {
                if entries.count() == 0 {
                    dirs_to_remove.push(path.to_path_buf());
                }
            }
        }
    }

    if files_to_remove.is_empty() && dirs_to_remove.is_empty() {
        println!("Nothing to clean");
        return;
    }

    let mut removed = 0;
    let mut freed = 0u64;

    for file in &files_to_remove {
        let size = fs::metadata(file).map(|m| m.len()).unwrap_or(0);
        if args.dry_run {
            println!("  [file] {}", file.display());
        } else {
            match fs::remove_file(file) {
                Ok(()) => {
                    println!("  [removed] {}", file.display());
                    removed += 1;
                    freed += size;
                }
                Err(e) => {
                    eprintln!("  [error] {}: {e}", file.display());
                }
            }
        }
    }

    for dir in &dirs_to_remove {
        if args.dry_run {
            println!("  [dir]  {}", dir.display());
        } else {
            match fs::remove_dir(dir) {
                Ok(()) => {
                    println!("  [removed] {}/", dir.display());
                    removed += 1;
                }
                Err(e) => {
                    eprintln!("  [error] {}/: {e}", dir.display());
                }
            }
        }
    }

    let action = if args.dry_run { "Would remove" } else { "Removed" };
    println!("\n  {action} {removed} items (freed {})", format_size(freed));
}

fn is_temp_file(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let name_lower = name.to_lowercase();

    let temp_patterns = [
        ".tmp", ".temp", ".bak", ".swp", ".swo", ".orig", ".old",
        ".cache", ".log", ".DS_Store", "Thumbs.db", "desktop.ini",
    ];

    for pattern in &temp_patterns {
        if name_lower.ends_with(pattern) || name_lower == pattern.trim_start_matches('.') {
            return true;
        }
    }

    let temp_dirs = [
        "__pycache__", ".pytest_cache", "node_modules", ".svn",
        ".git", ".idea", ".vscode", "target",
    ];

    for dir in &temp_dirs {
        if name_lower == *dir {
            return true;
        }
    }

    false
}

fn parse_duration(s: &str) -> std::time::Duration {
    let s = s.trim().to_lowercase();
    let (num_part, unit) = if let Some(pos) = s.find(|c: char| c.is_alphabetic()) {
        (&s[..pos], &s[pos..])
    } else {
        (s.as_str(), "d")
    };

    let num: u64 = num_part.parse().unwrap_or(0);
    let secs = match unit {
        "s" | "sec" | "second" | "seconds" => num,
        "m" | "min" | "minute" | "minutes" => num * 60,
        "h" | "hr" | "hour" | "hours" => num * 3600,
        "d" | "day" | "days" => num * 86400,
        "w" | "week" | "weeks" => num * 604800,
        "mo" | "month" | "months" => num * 2592000,
        "y" | "year" | "years" => num * 31536000,
        _ => num * 86400,
    };

    std::time::Duration::from_secs(secs)
}

fn parse_size(s: &str) -> Result<u64, String> {
    let s = s.trim().to_lowercase();
    let (num_part, unit) = if let Some(pos) = s.find(|c: char| c.is_alphabetic()) {
        (&s[..pos], &s[pos..])
    } else {
        (s.as_str(), "")
    };

    let num: u64 = num_part.parse().map_err(|_| format!("invalid number: {num_part}"))?;
    match unit {
        "" | "b" => Ok(num),
        "kb" | "k" => Ok(num * 1024),
        "mb" | "m" => Ok(num * 1024 * 1024),
        "gb" | "g" => Ok(num * 1024 * 1024 * 1024),
        _ => Err(format!("unknown unit: {unit}")),
    }
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
