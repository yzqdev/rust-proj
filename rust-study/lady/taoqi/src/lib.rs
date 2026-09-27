//! taoqi - JSON processor CLI (format, validate, minify, query).

use std::fs;
use std::path::PathBuf;

use clap::CommandFactory;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use serde_json::Value;

/// taoqi - JSON processor (format, validate, minify, query)
#[derive(Debug, Parser)]
#[command(
    name = "taoqi",
    version,
    author,
    about = "JSON processor CLI",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Pretty-print / format a JSON file
    Format {
        /// Path to the JSON file
        file: PathBuf,
        /// Write output back to the file (in-place)
        #[arg(short, long)]
        in_place: bool,
    },
    /// Validate a JSON file
    Validate {
        /// Path to the JSON file
        file: PathBuf,
    },
    /// Minify a JSON file (remove whitespace)
    Minify {
        /// Path to the JSON file
        file: PathBuf,
        /// Write output back to the file (in-place)
        #[arg(short, long)]
        in_place: bool,
    },
    /// Query a JSON value by key path
    Query {
        /// Path to the JSON file
        file: PathBuf,
        /// Key path, e.g. "name" or "address.city" (array items by index: "items.0")
        path: String,
    },
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
    /// Reading the input file failed.
    #[error("cannot read `{path}`: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// Writing the output file failed.
    #[error("cannot write `{path}`: {source}")]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// The input file is not valid JSON.
    #[error("\u{274c} `{path}` is NOT valid JSON: {source}")]
    InvalidJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },

    /// A key along the query path does not exist.
    #[error("key `{key}` not found in path `{path}`")]
    KeyNotFound { key: String, path: String },

    /// A non-numeric segment was used to index an array.
    #[error("expected array index, got `{0}`")]
    BadIndex(String),

    /// An array index is out of bounds.
    #[error("index {index} out of bounds (length {len})")]
    OutOfBounds { index: usize, len: usize },

    /// Tried to index into a scalar value.
    #[error("cannot index into a non-container value")]
    NotContainer,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Read and parse a JSON file.
pub fn load_json(path: &PathBuf) -> Result<Value> {
    let display = path.display().to_string();
    let content = fs::read_to_string(path).map_err(|source| Error::Io {
        path: display.clone(),
        source,
    })?;
    serde_json::from_str(&content).map_err(|source| Error::InvalidJson {
        path: display,
        source,
    })
}

/// Navigate a `Value` by a dotted key path; numeric segments index arrays.
pub fn query_value<'a>(value: &'a Value, path: &str) -> Result<&'a Value> {
    let mut current = value;
    for key in path.split('.') {
        match current {
            Value::Object(map) => match map.get(key) {
                Some(v) => current = v,
                None => {
                    return Err(Error::KeyNotFound {
                        key: key.to_string(),
                        path: path.to_string(),
                    });
                }
            },
            Value::Array(arr) => {
                let idx: usize = key.parse().map_err(|_| Error::BadIndex(key.to_string()))?;
                match arr.get(idx) {
                    Some(v) => current = v,
                    None => {
                        return Err(Error::OutOfBounds {
                            index: idx,
                            len: arr.len(),
                        });
                    }
                }
            }
            _ => return Err(Error::NotContainer),
        }
    }
    Ok(current)
}

/// Root type name of a JSON value (used by `validate`).
pub fn root_type(value: &Value) -> &'static str {
    match value {
        Value::Object(_) => "object",
        Value::Array(_) => "array",
        Value::String(_) => "string",
        Value::Number(_) => "number",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
    }
}

fn write_back(path: &PathBuf, content: &str) -> Result<()> {
    fs::write(path, content).map_err(|source| Error::Write {
        path: path.display().to_string(),
        source,
    })
}

/// Execute a parsed command and return the text to print on stdout.
pub fn run(cli: &Cli) -> Result<String> {
    match &cli.command {
        Commands::Format { file, in_place } => {
            let value = load_json(file)?;
            let formatted = serde_json::to_string_pretty(&value)
                .expect("serializing a parsed Value cannot fail");
            if *in_place {
                write_back(file, &formatted)?;
                Ok(format!("Formatted `{}` in-place", file.display()))
            } else {
                Ok(formatted)
            }
        }
        Commands::Validate { file } => {
            let value = load_json(file)?;
            Ok(format!(
                "\u{2705} `{}` is valid JSON (root type: {})",
                file.display(),
                root_type(&value)
            ))
        }
        Commands::Minify { file, in_place } => {
            let display = file.display().to_string();
            let content = fs::read_to_string(file).map_err(|source| Error::Io {
                path: display.clone(),
                source,
            })?;
            let value: Value =
                serde_json::from_str(&content).map_err(|source| Error::InvalidJson {
                    path: display.clone(),
                    source,
                })?;
            let minified =
                serde_json::to_string(&value).expect("serializing a parsed Value cannot fail");
            if *in_place {
                write_back(file, &minified)?;
                Ok(format!(
                    "Minified `{}` in-place ({} bytes -> {} bytes)",
                    display,
                    content.len(),
                    minified.len()
                ))
            } else {
                Ok(minified)
            }
        }
        Commands::Query { file, path } => {
            let value = load_json(file)?;
            let found = query_value(&value, path)?;
            Ok(
                serde_json::to_string_pretty(found)
                    .expect("serializing a parsed Value cannot fail"),
            )
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            let mut out = Vec::new();
            clap_complete::generate(*shell, &mut cmd, name, &mut out);
            Ok(String::from_utf8_lossy(&out).into_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn query_navigates_objects() {
        let v = json!({"name": "tq", "address": {"city": "Bj"}});
        assert_eq!(query_value(&v, "name").unwrap(), &json!("tq"));
        assert_eq!(query_value(&v, "address.city").unwrap(), &json!("Bj"));
    }

    #[test]
    fn query_indexes_arrays() {
        let v = json!({"items": [10, 20, 30]});
        assert_eq!(query_value(&v, "items.1").unwrap(), &json!(20));
    }

    #[test]
    fn query_reports_missing_keys() {
        let v = json!({"a": 1});
        let err = query_value(&v, "b").unwrap_err();
        assert!(err.to_string().contains("key `b` not found"));

        let err = query_value(&v, "a.x").unwrap_err();
        assert!(err.to_string().contains("non-container"));
    }

    #[test]
    fn query_reports_bad_array_indices() {
        let v = json!({"items": [1]});
        assert!(query_value(&v, "items.x").is_err());
        assert!(query_value(&v, "items.5").is_err());
    }

    #[test]
    fn root_type_detection() {
        assert_eq!(root_type(&json!({})), "object");
        assert_eq!(root_type(&json!([])), "array");
        assert_eq!(root_type(&json!("s")), "string");
        assert_eq!(root_type(&json!(1)), "number");
        assert_eq!(root_type(&json!(true)), "boolean");
        assert_eq!(root_type(&json!(null)), "null");
    }
}
