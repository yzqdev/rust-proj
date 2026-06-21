use std::fs;
use std::path::Path;

use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct InfoArgs {
    /// Files or directories to inspect
    #[arg(required = true, num_args = 1..)]
    pub paths: Vec<String>,

    /// Show detailed information
    #[arg(short, long)]
    pub detailed: bool,

    /// Show file type as MIME type
    #[arg(long)]
    pub mime: bool,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFmt::Pretty)]
    pub format: OutputFmt,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFmt {
    Pretty,
    Json,
}

pub fn execute(args: &InfoArgs) {
    for path_str in &args.paths {
        let path = Path::new(path_str);
        if !path.exists() {
            eprintln!("Error: '{path_str}' does not exist");
            continue;
        }

        let meta = match fs::metadata(path) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error reading '{path_str}': {e}");
                continue;
            }
        };

        match args.format {
            OutputFmt::Pretty => print_pretty(path, &meta, args),
            OutputFmt::Json => print_json(path, &meta, args),
        }
    }
}

fn print_pretty(path: &Path, meta: &fs::Metadata, args: &InfoArgs) {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
    let kind = if meta.is_dir() {
        "Directory"
    } else if meta.is_file() {
        "File"
    } else if meta.is_symlink() {
        "Symlink"
    } else {
        "Other"
    };

    println!("  Name:     {name}");
    println!("  Type:     {kind}");
    println!("  Size:     {} ({})", format_size(meta.len()), meta.len());

    if let Ok(modified) = meta.modified() {
        let dt: chrono::DateTime<chrono::Local> = modified.into();
        println!("  Modified: {}", dt.format("%Y-%m-%d %H:%M:%S"));
    }

    if let Ok(created) = meta.created() {
        let dt: chrono::DateTime<chrono::Local> = created.into();
        println!("  Created:  {}", dt.format("%Y-%m-%d %H:%M:%S"));
    }

    if args.detailed {
        println!("  Readonly: {}", !meta.permissions().readonly());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            println!("  Mode:     {:o}", meta.permissions().mode());
        }
    }

    if args.mime {
        if let Some(mime) = guess_mime(path) {
            println!("  MIME:     {mime}");
        }
    }

    if meta.is_dir() {
        let count = fs::read_dir(path)
            .map(|entries| entries.count())
            .unwrap_or(0);
        println!("  Entries:  {count}");
    }

    println!();
}

fn print_json(path: &Path, meta: &fs::Metadata, args: &InfoArgs) {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
    let kind = if meta.is_dir() {
        "directory"
    } else if meta.is_file() {
        "file"
    } else {
        "other"
    };

    let modified = meta.modified().ok().map(|t| {
        let dt: chrono::DateTime<chrono::Utc> = t.into();
        dt.to_rfc3339()
    });

    let created = meta.created().ok().map(|t| {
        let dt: chrono::DateTime<chrono::Utc> = t.into();
        dt.to_rfc3339()
    });

    let mime = if args.mime { guess_mime(path) } else { None };

    println!("{{");
    println!("  \"name\": \"{}\",", name);
    println!("  \"type\": \"{kind}\",");
    println!("  \"size\": {},", meta.len());
    println!("  \"readonly\": {},", !meta.permissions().readonly());
    if let Some(m) = modified {
        println!("  \"modified\": \"{m}\",");
    }
    if let Some(c) = created {
        println!("  \"created\": \"{c}\",");
    }
    if let Some(m) = mime {
        println!("  \"mime\": \"{m}\"");
    } else {
        println!("  \"mime\": null");
    }
    println!("}}");
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

fn guess_mime(path: &Path) -> Option<&'static str> {
    path.extension().and_then(|e| e.to_str()).map(|ext| match ext.to_lowercase().as_str() {
        "txt" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" | "mjs" => "application/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "yaml" | "yml" => "application/x-yaml",
        "toml" => "application/toml",
        "md" | "markdown" => "text/markdown",
        "rs" | "py" | "go" | "java" | "c" | "cpp" | "h" => "text/x-source",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",
        "pdf" => "application/pdf",
        _ => "application/octet-stream",
    })
}
