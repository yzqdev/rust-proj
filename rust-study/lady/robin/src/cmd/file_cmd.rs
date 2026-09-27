use std::fs;
use std::path::Path;

use colored::Colorize;

use crate::Error;
use crate::Result;

/// Compute and pretty-print the MD5 hash of a file.
pub fn calc_md5(file_path: &str) -> Result<String> {
    use std::time::Instant;
    let started = Instant::now();

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(Error::NotFound(file_path.to_string()));
    }

    let hash = hash_utils::file_hash(path, hash_utils::Algorithm::Md5)?;
    let file_size = fs::metadata(path)
        .map_err(|source| Error::Io {
            path: file_path.to_string(),
            source,
        })?
        .len();
    let elapsed = started.elapsed().as_secs_f64();

    let lines = [
        format!("File:     {file_path}").cyan().to_string(),
        format!("Size:     {}", file_size.to_string().yellow()),
        format!("MD5:      {hash}"),
        format!("Elapsed:  {elapsed}s"),
    ];
    Ok(lines.join("\n"))
}

/// Show basic image file information (type, size, permissions).
pub fn image_info(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);
    let metadata = metadata_of(path, file_path)?;

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("unknown")
        .to_lowercase();

    let is_image =
        ["png", "jpg", "jpeg", "gif", "bmp", "webp", "svg", "ico"].contains(&ext.as_str());

    let mut lines = Vec::new();
    if !is_image {
        lines.push(format!("Warning: '{file_path}' may not be an image file"));
    }
    lines.push(
        format!("=== Image Info: {file_path} ===")
            .green()
            .to_string(),
    );
    lines.push(format!("Type:     {}", ext.to_uppercase()));
    lines.push(format!("Size:     {} bytes", metadata.len()));
    lines.push(format!("Read-only: {}", metadata.permissions().readonly()));
    Ok(lines.join("\n"))
}

/// Show detailed file information.
pub fn file_info(file_path: &str) -> Result<String> {
    let path = Path::new(file_path);
    let metadata = metadata_of(path, file_path)?;

    let file_type = if metadata.is_dir() {
        "directory"
    } else if metadata.is_file() {
        "file"
    } else {
        "other"
    };

    let readonly = if metadata.permissions().readonly() {
        "yes".red().to_string()
    } else {
        "no".green().to_string()
    };
    let lines = [
        format!("=== File Info: {file_path} ===")
            .green()
            .to_string(),
        format!("Type:     {file_type}"),
        format!("Size:     {} bytes", metadata.len()),
        format!("Read-only: {readonly}"),
    ];
    Ok(lines.join("\n"))
}

/// Display a directory tree up to depth 3, skipping hidden entries.
pub fn dir_tree(dir_path: &str) -> Result<String> {
    let path = Path::new(dir_path);
    if !path.exists() || !path.is_dir() {
        return Err(Error::NotADirectory(dir_path.to_string()));
    }

    let mut out = String::new();
    out.push_str(&format!("Directory tree: {dir_path}\n").green().to_string());
    print_tree(path, 0, "", &mut out);
    Ok(out.trim_end().to_string())
}

fn metadata_of(path: &Path, display: &str) -> Result<fs::Metadata> {
    if !path.exists() {
        return Err(Error::NotFound(display.to_string()));
    }
    fs::metadata(path).map_err(|source| Error::Io {
        path: display.to_string(),
        source,
    })
}

fn print_tree(dir: &Path, depth: usize, prefix: &str, out: &mut String) {
    if depth > 3 {
        out.push_str(&format!("{prefix}   ... (max depth reached)\n"));
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut items: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .collect();

    items.sort_by_key(|e| e.file_name());

    let count = items.len();
    for (i, entry) in items.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let new_prefix = if is_last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();

        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            out.push_str(&format!("{prefix}{connector}{}/\n", name_str.cyan()));
            print_tree(&entry.path(), depth + 1, &new_prefix, out);
        } else {
            let size = entry.metadata().ok().map(|m| m.len()).unwrap_or(0);
            out.push_str(&format!(
                "{prefix}{connector}{} ({})\n",
                name_str,
                format_bytes(size)
            ));
        }
    }
}

/// Format a byte count with human-readable units.
pub fn format_bytes(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut s = size as f64;
    for unit in UNITS {
        if s < 1024.0 {
            return format!("{s:.1} {unit}");
        }
        s /= 1024.0;
    }
    format!("{s:.2} TB")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_formatting() {
        assert_eq!(format_bytes(0), "0.0 B");
        assert_eq!(format_bytes(512), "512.0 B");
        assert_eq!(format_bytes(2048), "2.0 KB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.0 MB");
        assert_eq!(format_bytes(3u64 * 1024 * 1024 * 1024), "3.0 GB");
        assert!(format_bytes(5u64 * 1024 * 1024 * 1024 * 1024).ends_with("TB"));
    }

    #[test]
    fn missing_file_is_reported() {
        let err = calc_md5("no-such-file.bin").unwrap_err();
        assert!(err.to_string().contains("no-such-file.bin"));
    }

    #[test]
    fn tree_requires_directory() {
        let err = dir_tree("no-such-dir/").unwrap_err();
        assert!(err.to_string().contains("not a valid directory"));
    }
}
