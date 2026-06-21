use std::fs;
use std::path::Path;

use clap::Args;

#[derive(Args)]
pub struct RenameArgs {
    /// Directory containing files to rename
    #[arg(required = true)]
    pub path: String,

    /// New name pattern. Use placeholders:
    ///   {n}   - incrementing number (1, 2, 3...)
    ///   {N}   - zero-padded number (001, 002...)
    ///   {name} - original filename without extension
    ///   {ext}  - original extension
    ///   {date} - current date (YYYY-MM-DD)
    #[arg(short, long)]
    pub pattern: String,

    /// Change file extension
    #[arg(short, long)]
    pub ext: Option<String>,

    /// Starting number for {n}/{N}
    #[arg(short, long, default_value_t = 1)]
    pub start: u32,

    /// Zero-pad width for {N}
    #[arg(short = 'w', long, default_value_t = 3)]
    pub pad_width: usize,

    /// Dry run (show what would be renamed)
    #[arg(long)]
    pub dry_run: bool,

    /// Only rename files matching this extension
    #[arg(long)]
    pub filter_ext: Option<String>,

    /// Recurse into subdirectories
    #[arg(short, long)]
    pub recursive: bool,
}

pub fn execute(args: &RenameArgs) {
    let dir = Path::new(&args.path);
    if !dir.is_dir() {
        eprintln!("Error: '{}' is not a directory", args.path);
        return;
    }

    let mut entries: Vec<_> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter(|e| {
            if let Some(ref ext) = args.filter_ext {
                e.path()
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case(ext))
                    .unwrap_or(false)
            } else {
                true
            }
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let mut counter = args.start;

    for entry in &entries {
        let path = entry.path();
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let new_ext = args.ext.as_deref().unwrap_or(ext);
        let padded = format!("{:0>width$}", counter, width = args.pad_width);
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();

        let new_name = args
            .pattern
            .replace("{n}", &counter.to_string())
            .replace("{N}", &padded)
            .replace("{name}", stem)
            .replace("{ext}", ext)
            .replace("{date}", &date);

        let new_path = path.with_file_name(format!("{new_name}.{new_ext}"));

        if new_path == path {
            counter += 1;
            continue;
        }

        if args.dry_run {
            println!(
                "  {} -> {}",
                path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                new_path.file_name().and_then(|n| n.to_str()).unwrap_or("?")
            );
        } else {
            match fs::rename(&path, &new_path) {
                Ok(()) => {
                    println!(
                        "  {} -> {}",
                        path.file_name().and_then(|n| n.to_str()).unwrap_or("?"),
                        new_path.file_name().and_then(|n| n.to_str()).unwrap_or("?")
                    );
                }
                Err(e) => {
                    eprintln!("  Error renaming '{}': {e}", path.display());
                }
            }
        }

        counter += 1;
    }

    let action = if args.dry_run { "Would rename" } else { "Renamed" };
    println!("\n  {action} {} files", counter - args.start);
}
