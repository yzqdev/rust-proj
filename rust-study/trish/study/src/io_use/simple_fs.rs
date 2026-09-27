use std::fs;
use std::path::Path;

use md5::Digest;

/// Compute MD5 hash of a file, returning hex string.
pub fn compute_md5(path: &str) -> Result<String, std::io::Error> {
    let data = fs::read(path)?;
    Ok(format!("{:x}", md5::Md5::digest(data)))
}

pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// Get a human-readable file size string.
pub fn file_size(path: &str) -> Result<String, std::io::Error> {
    let size = fs::metadata(path)?.len();
    let size_str = if size < 1024 {
        format!("{size} B")
    } else if size < 1024 * 1024 {
        format!("{:.2} KB", size as f64 / 1024.0)
    } else {
        format!("{:.2} MB", size as f64 / (1024.0 * 1024.0))
    };
    Ok(format!("Size of '{path}': {size_str} ({size})"))
}

/// List directory contents (dirs first, then files).
pub fn list_dir(path: &str) -> Result<String, std::io::Error> {
    let entries = fs::read_dir(path)?;

    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            dirs.push(format!("[DIR]  {name}"));
        } else {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            files.push(format!("[FILE] {name} ({size} bytes)"));
        }
    }
    dirs.sort();
    files.sort();

    let mut out = format!("Contents of '{path}':\n");
    for d in &dirs {
        out.push_str(&format!("  {d}\n"));
    }
    for f in &files {
        out.push_str(&format!("  {f}\n"));
    }
    out.push_str(&format!("Total: {} entries", dirs.len() + files.len()));
    Ok(out)
}

/// Check if a file exists.
pub fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn file_size_reports() {
        let mut path = std::env::temp_dir();
        path.push(format!("study-size-{}.txt", std::process::id()));
        fs::write(&path, b"12345678").unwrap();

        let out = file_size(path.to_str().unwrap()).unwrap();
        assert!(out.contains("8 B"), "{out}");
        fs::remove_file(&path).ok();
    }

    #[test]
    fn file_size_missing_file_is_error() {
        assert!(file_size("no-such-file.xyz").is_err());
    }

    #[test]
    fn list_dir_reports_entries() {
        let dir = std::env::temp_dir().join(format!("study-ls-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("a.txt"), b"a").unwrap();
        fs::create_dir_all(dir.join("sub")).unwrap();

        let out = list_dir(dir.to_str().unwrap()).unwrap();
        assert!(out.contains("[DIR]  sub"), "{out}");
        assert!(out.contains("[FILE] a.txt (1 bytes)"), "{out}");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn compute_md5_known_vector() {
        let mut path = std::env::temp_dir();
        path.push(format!("study-md5-{}.txt", std::process::id()));
        fs::write(&path, b"hello").unwrap();
        assert_eq!(
            compute_md5(path.to_str().unwrap()).unwrap(),
            "5d41402abc4b2a76b9719d911017c592"
        );
        fs::remove_file(&path).ok();
    }
}
