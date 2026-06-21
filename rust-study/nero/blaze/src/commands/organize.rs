use std::fs;
use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};
use walkdir::WalkDir;

#[derive(Args)]
pub struct OrganizeArgs {
    /// Directory to organize
    #[arg(required = true)]
    pub path: String,

    /// Organize by criteria
    #[arg(short, long, value_enum, default_value_t = OrganizeBy::Extension)]
    pub by: OrganizeBy,

    /// Dry run (show what would be moved)
    #[arg(long)]
    pub dry_run: bool,

    /// Recurse into subdirectories
    #[arg(short, long)]
    pub recursive: bool,

    /// Move instead of copy
    #[arg(short, long)]
    pub r#move: bool,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OrganizeBy {
    /// Group by file extension
    Extension,
    /// Group by modification date (YYYY-MM)
    Date,
    /// Group by size (small/medium/large/huge)
    Size,
}

pub fn execute(args: &OrganizeArgs) {
    let root = Path::new(&args.path);
    if !root.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.path);
        return;
    }

    let files: Vec<PathBuf> = WalkDir::new(root)
        .max_depth(if args.recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect();

    if files.is_empty() {
        println!("No files found to organize");
        return;
    }

    let mut moved = 0;
    let mut skipped = 0;

    for file in &files {
        let category = match args.by {
            OrganizeBy::Extension => {
                file.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("no_extension")
                    .to_lowercase()
            }
            OrganizeBy::Date => {
                file.metadata()
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .map(|t| {
                        let dt: chrono::DateTime<chrono::Local> = t.into();
                        dt.format("%Y-%m").to_string()
                    })
                    .unwrap_or_else(|| "unknown-date".to_string())
            }
            OrganizeBy::Size => {
                let size = file.metadata().map(|m| m.len()).unwrap_or(0);
                if size < 1024 {
                    "tiny".to_string()
                } else if size < 1024 * 1024 {
                    "small".to_string()
                } else if size < 1024 * 1024 * 1024 {
                    "medium".to_string()
                } else {
                    "large".to_string()
                }
            }
        };

        let dest_dir = root.join(&category);
        let file_name = file.file_name().unwrap();
        let dest = dest_dir.join(file_name);

        if dest == *file {
            skipped += 1;
            continue;
        }

        if args.dry_run {
            println!("  {} -> {}", file.display(), dest.display());
        } else {
            if !dest_dir.exists() {
                if let Err(e) = fs::create_dir_all(&dest_dir) {
                    eprintln!("  Error creating '{}': {e}", dest_dir.display());
                    continue;
                }
            }

            let result = if args.r#move {
                fs::rename(file, &dest)
            } else {
                fs::copy(file, &dest).map(|_| ())
            };

            match result {
                Ok(()) => {
                    println!("  {} -> {}", file.display(), dest.display());
                    moved += 1;
                }
                Err(e) => {
                    eprintln!("  Error: {e}");
                }
            }
        }
    }

    let action = if args.dry_run { "Would organize" } else { "Organized" };
    println!("\n  {action} {moved} files (skipped {skipped})");
}
