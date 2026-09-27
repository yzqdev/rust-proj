//! Shared streaming hash helpers used by the CLIs in this workspace.
//!
//! The module keeps the hashing logic (chunked file reads, hex encoding) in
//! one place so that `asta` and `robin` do not duplicate it.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use digest::Digest;
use md5::Md5;
use sha2::Sha256;

/// Hash algorithms supported by [`file_hash`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Md5,
    Sha256,
}

/// Errors that can occur while hashing.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The given path does not exist or is not a regular file.
    #[error("file not found: {0}")]
    NotFound(String),

    /// Opening or reading the file failed.
    #[error("failed to read `{path}`: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

/// Result type for hashing operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Compute the MD5 digest of a byte slice, hex encoded (lowercase).
pub fn md5_hex(data: &[u8]) -> String {
    hex(Md5::digest(data))
}

/// Compute the SHA256 digest of a byte slice, hex encoded (lowercase).
pub fn sha256_hex(data: &[u8]) -> String {
    hex(Sha256::digest(data))
}

/// Compute the digest of a file, reading it in 8 KiB chunks.
pub fn file_hash(path: &Path, algorithm: Algorithm) -> Result<String> {
    let path_str = path.display().to_string();
    if !path.exists() {
        return Err(Error::NotFound(path_str));
    }

    let mut file = File::open(path).map_err(|source| Error::Io {
        path: path_str.clone(),
        source,
    })?;

    match algorithm {
        Algorithm::Md5 => hash_stream(&mut file, Md5::new()).map_err(|source| Error::Io {
            path: path_str,
            source,
        }),
        Algorithm::Sha256 => hash_stream(&mut file, Sha256::new()).map_err(|source| Error::Io {
            path: path_str,
            source,
        }),
    }
}

fn hash_stream<D: Digest>(file: &mut File, mut hasher: D) -> std::io::Result<String> {
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(hex(hasher.finalize()))
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const MD5_HELLO: &str = "5d41402abc4b2a76b9719d911017c592";
    const SHA256_HELLO: &str = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";

    #[test]
    fn md5_of_hello() {
        assert_eq!(md5_hex(b"hello"), MD5_HELLO);
    }

    #[test]
    fn sha256_of_hello() {
        assert_eq!(sha256_hex(b"hello"), SHA256_HELLO);
    }

    #[test]
    fn file_hash_matches_text_hash() {
        let mut path = std::env::temp_dir();
        path.push(format!("hash-utils-test-{}.txt", std::process::id()));
        let mut f = File::create(&path).unwrap();
        f.write_all(b"hello").unwrap();
        drop(f);

        assert_eq!(file_hash(&path, Algorithm::Md5).unwrap(), MD5_HELLO);
        assert_eq!(file_hash(&path, Algorithm::Sha256).unwrap(), SHA256_HELLO);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn missing_file_is_reported() {
        let err = file_hash(Path::new("definitely-missing-file.xyz"), Algorithm::Md5).unwrap_err();
        assert!(matches!(err, Error::NotFound(_)));
    }

    #[test]
    fn error_display_is_user_friendly() {
        let err = Error::NotFound("foo.bin".into());
        assert_eq!(err.to_string(), "file not found: foo.bin");
    }
}
