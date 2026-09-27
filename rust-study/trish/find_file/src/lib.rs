//! find_file - File finder library.
//!
//! Helpers used by the CLI: glob search (normalized for Windows), size /
//! recency / emptiness filters, and simple line search.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use glob::glob;

/// Build a glob pattern from a base path, normalizing `\` to `/` (the glob
/// crate uses `/` as the separator and treats `\` as an escape character).
pub fn build_pattern(base: &str, suffix: &str) -> String {
    format!("{}/{}/{}", base.replace('\\', "/"), "**", suffix)
}

/// Find files matching a glob pattern under `path`.
pub fn find_by_pattern(path: &str, extension: &str) -> Vec<PathBuf> {
    let pattern = build_pattern(path, &format!("*.{extension}"));
    match glob(&pattern) {
        Ok(paths) => paths.flatten().collect(),
        Err(_) => Vec::new(),
    }
}

/// Find files strictly larger than `min_size` bytes.
pub fn find_large(path: &str, min_size: u64) -> Vec<(PathBuf, u64)> {
    let pattern = build_pattern(path, "*");
    let mut results = Vec::new();
    if let Ok(paths) = glob(&pattern) {
        for entry in paths.flatten() {
            if let Ok(meta) = fs::metadata(&entry)
                && meta.len() > min_size
            {
                results.push((entry, meta.len()));
            }
        }
    }
    results
}

/// Find files modified within the last `days` days.
pub fn find_recent(path: &str, days: u64) -> Vec<PathBuf> {
    let pattern = build_pattern(path, "*");
    let threshold = SystemTime::now() - Duration::from_secs(days * 24 * 3600);
    let mut results = Vec::new();
    if let Ok(paths) = glob(&pattern) {
        for entry in paths.flatten() {
            if let Ok(meta) = fs::metadata(&entry)
                && let Ok(modified) = meta.modified()
                && modified > threshold
            {
                results.push(entry);
            }
        }
    }
    results
}

/// Find empty files and directories.
pub fn find_empty(path: &str) -> Vec<(PathBuf, &'static str)> {
    let pattern = build_pattern(path, "*");
    let mut results = Vec::new();
    if let Ok(paths) = glob(&pattern) {
        for entry in paths.flatten() {
            if entry.is_dir() {
                let empty = entry
                    .read_dir()
                    .map(|mut d| d.next().is_none())
                    .unwrap_or(false);
                if empty {
                    results.push((entry, "dir"));
                }
            } else if let Ok(meta) = fs::metadata(&entry)
                && meta.len() == 0
            {
                results.push((entry, "file"));
            }
        }
    }
    results
}

/// Return the lines of `contents` containing `query`.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("find-file-{}-{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("nested")).unwrap();
        fs::write(dir.join("a.png"), b"pngdata").unwrap();
        fs::write(dir.join("b.txt"), b"content\nwith query line\nend").unwrap();
        fs::write(dir.join("nested").join("c.png"), b"x").unwrap();
        fs::write(dir.join("big.bin"), vec![0u8; 2048]).unwrap();
        fs::write(dir.join("empty.txt"), b"").unwrap();
        fs::create_dir_all(dir.join("empty_dir")).unwrap();
        dir
    }

    #[test]
    fn pattern_finds_files_recursively() {
        let dir = setup("pattern");
        let found = find_by_pattern(dir.to_str().unwrap(), "png");
        assert_eq!(found.len(), 2, "{found:?}");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn large_finds_big_files_only() {
        let dir = setup("large");
        let found = find_large(dir.to_str().unwrap(), 1024);
        assert_eq!(found.len(), 1);
        assert!(found[0].0.ends_with("big.bin"));
        assert_eq!(found[0].1, 2048);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn recent_finds_new_files() {
        let dir = setup("recent");
        let found = find_recent(dir.to_str().unwrap(), 1);
        assert!(found.len() >= 4, "{found:?}");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn empty_finds_files_and_dirs() {
        let dir = setup("empty");
        let found = find_empty(dir.to_str().unwrap());
        let files = found.iter().filter(|(_, k)| *k == "file").count();
        let dirs = found.iter().filter(|(_, k)| *k == "dir").count();
        assert_eq!(files, 1, "{found:?}");
        assert_eq!(dirs, 1, "{found:?}");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn search_finds_matching_lines() {
        let contents = "alpha\nbeta query\ngamma\nquery again";
        let lines = search("query", contents);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "beta query");
    }

    #[test]
    fn pattern_normalizes_windows_separators() {
        let p = build_pattern("C:\\data\\logs", "*.txt");
        assert!(p.starts_with("C:/data/logs/"));
        assert!(p.ends_with("**/*.txt"));
    }
}
