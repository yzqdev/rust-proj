//! sampo - A fictional versioning CLI (clap **builder API** variant).
//!
//! Sister project of `pela`, which implements the same CLI with the clap
//! **derive API**. Both are kept as teaching examples of the two styles.
//! Operations are simulated; `init` and `clone` do touch the filesystem.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Errors reported by the CLI layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A staged path does not exist.
    #[error("path not found: {}", .0.display())]
    PathNotFound(PathBuf),

    /// A required argument was missing at dispatch time (should not happen
    /// for arguments clap marks as required).
    #[error("missing required argument `{0}`")]
    MissingArgument(String),

    /// Filesystem operation failed.
    #[error("{action}: {source}")]
    Io {
        action: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Initialize a new repository directory (`.<name>/README.md`).
pub fn init_repo(name: &str) -> Result<String> {
    let dir = PathBuf::from(format!(".{name}"));
    fs::create_dir_all(&dir).map_err(|source| Error::Io {
        action: "failed to create repository directory",
        source,
    })?;
    let mut readme = fs::File::create(dir.join("README.md")).map_err(|source| Error::Io {
        action: "failed to create repository README",
        source,
    })?;
    writeln!(readme, "# {name}").map_err(|source| Error::Io {
        action: "failed to write repository README",
        source,
    })?;
    Ok(format!("Initialized empty repository: {}", dir.display()))
}

/// Clone a remote into a directory derived from the URL.
pub fn clone_repo(remote: &str) -> Result<String> {
    let dir_name = remote
        .split('/')
        .next_back()
        .unwrap_or(remote)
        .trim_end_matches(".git");
    let dir = Path::new(dir_name);
    fs::create_dir_all(dir).map_err(|source| Error::Io {
        action: "failed to create clone directory",
        source,
    })?;
    Ok(format!("Cloned '{remote}' into '{dir_name}'"))
}

/// Report staging result for the given paths.
pub fn add_paths(paths: &[PathBuf]) -> Result<String> {
    let mut lines = Vec::new();
    for p in paths {
        if !p.exists() {
            return Err(Error::PathNotFound(p.clone()));
        }
        lines.push(format!("Staged: {}", p.display()));
    }
    Ok(lines.join("\n"))
}

/// Simulated commit.
pub fn commit(message: &str) -> String {
    format!("Committed with message: \"{message}\"")
}

/// Simulated status output.
pub fn status() -> String {
    "On branch main\nNothing to commit, working tree clean".into()
}

/// Simulated push.
pub fn push(remote: &str) -> String {
    format!("Pushing to '{remote}' (simulated)")
}

/// Simulated diff report.
pub fn diff(base: &str, head: &str, path: &str, color: &str) -> String {
    format!("Diffing {base}..{head} {path} (color={color})")
}

/// Simulated log output.
pub fn log(max_count: &str) -> String {
    format!(
        "Showing last {max_count} commits (simulated)\ncommit a1b2c3d4e5f6... (HEAD -> main)\n    Initial commit"
    )
}

/// External subcommand passthrough report.
pub fn external(command: &str, args: &[std::ffi::OsString]) -> String {
    format!("Calling out to {command:?} with {args:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_directory_and_readme() {
        let dir = std::env::temp_dir().join(format!("sampo-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let prev = std::env::current_dir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let out = init_repo("demo").unwrap();
        assert!(out.contains(".demo"));
        assert!(dir.join(".demo").join("README.md").exists());

        std::env::set_current_dir(prev).unwrap();
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn clone_derives_directory_name() {
        assert_eq!(
            clone_repo("https://example.com/group/repo.git").unwrap(),
            "Cloned 'https://example.com/group/repo.git' into 'repo'"
        );
    }

    #[test]
    fn add_missing_path_is_an_error() {
        let err = add_paths(&[PathBuf::from("definitely-missing.txt")]).unwrap_err();
        assert!(err.to_string().contains("path not found"));
    }

    #[test]
    fn status_reports_clean_tree() {
        assert!(status().contains("working tree clean"));
    }
}
