use std::fs;

use md5::Digest;
use rand::Rng;

/// Errors reported by the CLI layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reading a file failed.
    #[error("cannot read `{path}`: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// An HTTP request failed.
    #[error("request to `{url}` failed: {source}")]
    Request {
        url: String,
        #[source]
        source: reqwest::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// MD5 hash of a string, hex encoded.
pub fn hash_md5(text: &str) -> String {
    format!("{:x}", md5::Md5::digest(text.as_bytes()))
}

/// Random number in `[min, max]` (inclusive).
///
/// Returns `min` when `min > max` instead of panicking.
pub fn random_in_range(min: i32, max: i32) -> i32 {
    if min >= max {
        return min;
    }
    rand::thread_rng().gen_range(min..=max)
}

/// Read a text file.
pub fn read_text(path: &str) -> Result<String> {
    fs::read_to_string(path).map_err(|source| Error::Io {
        path: path.to_string(),
        source,
    })
}

/// System information summary.
pub fn info() -> String {
    format!(
        "=== System Info ===\nOS: {}\nArch: {}\nCurrent dir: {:?}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::current_dir().unwrap_or_default()
    )
}

/// Fetch a URL and return a status + body preview report.
pub async fn fetch(url: &str) -> Result<String> {
    let resp = reqwest::get(url).await.map_err(|source| Error::Request {
        url: url.to_string(),
        source,
    })?;
    let status = resp.status();
    let body = resp.text().await.map_err(|source| Error::Request {
        url: url.to_string(),
        source,
    })?;
    let preview: String = body.chars().take(500).collect();
    Ok(format!(
        "Status: {status}\nBody (first 500 chars):\n{preview}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md5_known_vector() {
        assert_eq!(hash_md5("hello"), "5d41402abc4b2a76b9719d911017c592");
    }

    #[test]
    fn random_in_bounds() {
        for _ in 0..100 {
            let n = random_in_range(1, 100);
            assert!((1..=100).contains(&n));
        }
    }

    #[test]
    fn random_swapped_bounds_do_not_panic() {
        assert_eq!(random_in_range(10, 1), 10);
    }
}
