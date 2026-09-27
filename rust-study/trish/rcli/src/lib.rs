//! rcli - A general-purpose CLI (base64, password generation, word count).

pub mod util;

use std::fs;
use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Encode text to base64
    Encode {
        /// Text to encode
        text: String,
    },
    /// Decode base64 text
    Decode {
        /// Base64 text to decode
        text: String,
    },
    /// Generate a random password
    Genpwd {
        /// Length of the password
        #[arg(short, long, default_value_t = 16)]
        length: u8,
        /// Include special characters
        #[arg(short, long)]
        special: bool,
    },
    /// Count lines, words, and chars in text or file
    Count {
        /// Text or file path
        input: String,
        /// Treat input as file path
        #[arg(short, long)]
        file: bool,
    },
    /// Calculate time benchmark
    Bench,
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
}

/// Errors reported by the CLI layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The input is not valid base64.
    #[error("{0}")]
    Base64(String),

    /// Reading a `--file` input failed.
    #[error("cannot read `{path}`: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

/// Execute a parsed command and return the text to print on stdout.
pub fn run(cli: &Cli) -> Result<String> {
    let mut lines: Vec<String> = Vec::new();

    if let Some(name) = cli.name.as_deref() {
        lines.push(format!("Value for name: {name}"));
    }
    if let Some(config_path) = cli.config.as_deref() {
        lines.push(format!("Value for config: {}", config_path.display()));
    }
    lines.push(match cli.debug {
        0 => "Debug mode is off".into(),
        1 => "Debug mode is kind of on".into(),
        2 => "Debug mode is on".into(),
        _ => "Don't be crazy".into(),
    });

    match &cli.command {
        Some(Commands::Test { list }) => {
            lines.push(if *list {
                "Printing testing lists...".into()
            } else {
                "Not printing testing lists...".into()
            });
        }
        Some(Commands::Encode { text }) => {
            lines.push(format!("Encoded: {}", base64_encode(text)));
        }
        Some(Commands::Decode { text }) => {
            lines.push(format!("Decoded: {}", base64_decode(text)?));
        }
        Some(Commands::Genpwd { length, special }) => {
            lines.push(format!("Generated password: {}", genpwd(*length, *special)));
        }
        Some(Commands::Count { input, file }) => {
            let content = if *file {
                fs::read_to_string(input).map_err(|source| Error::Io {
                    path: input.clone(),
                    source,
                })?
            } else {
                input.clone()
            };
            lines.push(count(&content));
        }
        Some(Commands::Bench) => {
            util::cal_time::cal_time();
        }
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            return Ok(String::from_utf8_lossy(&out).into_owned());
        }
        None => {}
    }

    Ok(lines.join("\n"))
}

/// Count lines, whitespace-separated words and chars.
pub fn count(content: &str) -> String {
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    format!("Lines: {lines}, Words: {words}, Chars: {chars}")
}

/// Generate a random password of the given length.
pub fn genpwd(length: u8, special: bool) -> String {
    use rand::Rng;

    const ALNUM: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const SPECIAL: &str = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~";
    let chars: String = if special {
        format!("{ALNUM}{SPECIAL}")
    } else {
        ALNUM.to_string()
    };

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars.as_bytes()[idx] as char
        })
        .collect()
}

const BASE64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Encode bytes as base64 (standard alphabet, padded).
pub fn base64_encode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let combined = (b0 << 16) | (b1 << 8) | b2;

        result.push(BASE64_ALPHABET[((combined >> 18) & 0x3F) as usize] as char);
        result.push(BASE64_ALPHABET[((combined >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(BASE64_ALPHABET[((combined >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_ALPHABET[(combined & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

/// Decode base64 text, rejecting invalid characters.
pub fn base64_decode(input: &str) -> Result<String> {
    const DECODE: [i8; 128] = {
        let mut table = [-1i8; 128];
        let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut i = 0;
        while i < 64 {
            table[chars[i] as usize] = i as i8;
            i += 1;
        }
        table
    };

    let bytes: Vec<u8> = input
        .bytes()
        .filter(|&b| b != b'\n' && b != b'\r' && b != b' ')
        .collect();

    let padding = bytes.iter().filter(|&&b| b == b'=').count();
    let valid_bytes = bytes.len() - padding;

    if !bytes[valid_bytes..].iter().all(|&b| b == b'=') {
        return Err(Error::Base64("padding must be at the end".into()));
    }

    let mut result = Vec::new();

    for chunk in bytes[..valid_bytes].chunks(4) {
        if chunk.len() == 1 {
            return Err(Error::Base64("invalid base64 length".into()));
        }
        let mut buf = [0u8; 4];
        for (i, &b) in chunk.iter().enumerate() {
            if b as usize >= 128 || DECODE[b as usize] == -1 {
                return Err(Error::Base64(format!(
                    "invalid base64 character: {}",
                    b as char
                )));
            }
            buf[i] = DECODE[b as usize] as u8;
        }

        let combined =
            (buf[0] as u32) << 18 | (buf[1] as u32) << 12 | (buf[2] as u32) << 6 | (buf[3] as u32);
        result.push((combined >> 16) as u8);
        if chunk.len() > 2 {
            result.push((combined >> 8) as u8);
        }
        if chunk.len() > 3 {
            result.push(combined as u8);
        }
    }

    String::from_utf8(result).map_err(|e| Error::Base64(format!("UTF-8 error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_known_vectors() {
        assert_eq!(base64_encode(""), "");
        assert_eq!(base64_encode("f"), "Zg==");
        assert_eq!(base64_encode("fo"), "Zm8=");
        assert_eq!(base64_encode("foo"), "Zm9v");
        assert_eq!(base64_encode("foobar"), "Zm9vYmFy");
    }

    #[test]
    fn base64_roundtrip() {
        for text in ["hello", "rust 🦀 CLI", "line1\nline2"] {
            assert_eq!(base64_decode(&base64_encode(text)).unwrap(), text);
        }
    }

    #[test]
    fn base64_rejects_invalid() {
        assert!(base64_decode("ab$c").is_err());
    }

    #[test]
    fn count_counts_lines_words_chars() {
        assert_eq!(count("one two\nthree"), "Lines: 2, Words: 3, Chars: 13");
    }

    #[test]
    fn genpwd_respects_length_and_alphabet() {
        let pwd = genpwd(24, false);
        assert_eq!(pwd.len(), 24);
        assert!(pwd.chars().all(|c| c.is_ascii_alphanumeric()));

        let pwd = genpwd(24, true);
        assert_eq!(pwd.len(), 24);
    }
}
