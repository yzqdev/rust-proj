use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use clap::{Args, ValueEnum};
use walkdir::WalkDir;

#[derive(Args)]
pub struct FindArgs {
    /// Directory to search in
    #[arg(default_value = ".")]
    pub path: String,

    /// Filter by file extension (without dot)
    #[arg(short, long)]
    pub ext: Option<String>,

    /// Filter by filename pattern (substring match)
    #[arg(short = 'p', long)]
    pub pattern: Option<String>,

    /// Filter by minimum file size (e.g., 1kb, 5mb, 1gb)
    #[arg(long)]
    pub min_size: Option<String>,

    /// Filter by maximum file size
    #[arg(long)]
    pub max_size: Option<String>,

    /// Only include files modified after this date (YYYY-MM-DD)
    #[arg(long)]
    pub after: Option<String>,

    /// Only include files modified before this date
    #[arg(long)]
    pub before: Option<String>,

    /// Maximum search depth (0 = unlimited)
    #[arg(short, long, default_value_t = 0)]
    pub depth: usize,

    /// Show results as full paths
    #[arg(long)]
    pub full_path: bool,

    /// Sort results
    #[arg(long, value_enum, default_value_t = SortBy::Name)]
    pub sort: SortBy,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFmt::Pretty)]
    pub format: OutputFmt,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum SortBy {
    Name,
    Size,
    Modified,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFmt {
    Pretty,
    Csv,
    Json,
}

pub fn execute(args: &FindArgs) {
    let root = PathBuf::from(&args.path);
    if !root.exists() {
        eprintln!("Error: path '{}' does not exist", args.path);
        return;
    }

    let min_bytes = args.min_size.as_deref().map(parse_size).transpose();
    let max_bytes = args.max_size.as_deref().map(parse_size).transpose();

    if let Err(e) = min_bytes {
        eprintln!("Error parsing --min-size: {e}");
        return;
    }
    if let Err(e) = max_bytes {
        eprintln!("Error parsing --max-size: {e}");
        return;
    }

    let min_bytes = min_bytes.unwrap();
    let max_bytes = max_bytes.unwrap();

    let mut entries: Vec<(PathBuf, fs::Metadata)> = WalkDir::new(&root)
        .max_depth(if args.depth == 0 { usize::MAX } else { args.depth })
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            Some((e.into_path(), meta))
        })
        .filter(|(path, _)| {
            if let Some(ref ext) = args.ext {
                path.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case(ext))
                    .unwrap_or(false)
            } else {
                true
            }
        })
        .filter(|(path, _)| {
            if let Some(ref pattern) = args.pattern {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.contains(pattern.as_str()))
                    .unwrap_or(false)
            } else {
                true
            }
        })
        .filter(|(_, meta)| {
            min_bytes.map_or(true, |min| meta.len() >= min)
        })
        .filter(|(_, meta)| {
            max_bytes.map_or(true, |max| meta.len() <= max)
        })
        .collect();

    entries.sort_by(|a, b| match args.sort {
        SortBy::Name => a.0.file_name().cmp(&b.0.file_name()),
        SortBy::Size => a.1.len().cmp(&b.1.len()),
        SortBy::Modified => a.1.modified().unwrap_or(SystemTime::UNIX_EPOCH)
            .cmp(&b.1.modified().unwrap_or(SystemTime::UNIX_EPOCH)),
    });

    match args.format {
        OutputFmt::Pretty => {
            for (path, meta) in &entries {
                let display = if args.full_path {
                    path.display().to_string()
                } else {
                    path.strip_prefix(&root)
                        .unwrap_or(path)
                        .display()
                        .to_string()
                };
                let size = format_size(meta.len());
                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("-");
                println!("{size:>10}  {ext:<8}  {display}");
            }
            println!("\nFound {} files", entries.len());
        }
        OutputFmt::Csv => {
            println!("path,size,extension,modified");
            for (path, meta) in &entries {
                let display = path.display();
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let modified = meta.modified()
                    .ok()
                    .and_then(|t| {
                        let dt: chrono::DateTime<chrono::Local> = t.into();
                        Some(dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    })
                    .unwrap_or_default();
                println!("{display},{},{ext},{modified}", meta.len());
            }
        }
        OutputFmt::Json => {
            println!("[");
            for (i, (path, meta)) in entries.iter().enumerate() {
                let comma = if i < entries.len() - 1 { "," } else { "" };
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                println!(
                    "  {{\"path\":\"{}\",\"size\":{},\"extension\":\"{}\"}}{}",
                    path.display().to_string().replace('\\', "/"),
                    meta.len(),
                    ext,
                    comma
                );
            }
            println!("]");
        }
    }
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
        "tb" | "t" => Ok(num * 1024 * 1024 * 1024 * 1024),
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
