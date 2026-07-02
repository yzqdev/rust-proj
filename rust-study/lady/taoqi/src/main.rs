use std::fs;

use clap::Parser;

/// taoqi - JSON processor (format, validate, minify, query)
#[derive(Parser)]
#[command(name = "taoqi", version, author, about = "JSON processor CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Pretty-print / format a JSON file
    Format {
        /// Path to the JSON file
        file: String,
        /// Write output back to the file (in-place)
        #[arg(short, long)]
        in_place: bool,
    },
    /// Validate a JSON file
    Validate {
        /// Path to the JSON file
        file: String,
    },
    /// Minify a JSON file (remove whitespace)
    Minify {
        /// Path to the JSON file
        file: String,
        /// Write output back to the file (in-place)
        #[arg(short, long)]
        in_place: bool,
    },
    /// Query a JSON value by key path
    Query {
        /// Path to the JSON file
        file: String,
        /// Key path, e.g. "name" or "address.city"
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Format { file, in_place } => {
            let content = read_file(&file);
            let value: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error: invalid JSON in '{}': {}", file, e);
                    std::process::exit(1);
                }
            };
            let formatted = serde_json::to_string_pretty(&value).unwrap();
            if in_place {
                fs::write(&file, &formatted).unwrap_or_else(|e| {
                    eprintln!("Error: cannot write '{}': {}", file, e);
                    std::process::exit(1);
                });
                println!("Formatted '{}' in-place", file);
            } else {
                println!("{}", formatted);
            }
        }
        Commands::Validate { file } => {
            let content = read_file(&file);
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(v) => {
                    let kind = match v {
                        serde_json::Value::Object(_) => "object",
                        serde_json::Value::Array(_) => "array",
                        serde_json::Value::String(_) => "string",
                        serde_json::Value::Number(_) => "number",
                        serde_json::Value::Bool(_) => "boolean",
                        serde_json::Value::Null => "null",
                    };
                    println!("✅ '{}' is valid JSON (root type: {})", file, kind);
                }
                Err(e) => {
                    eprintln!("❌ '{}' is NOT valid JSON: {}", file, e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Minify { file, in_place } => {
            let content = read_file(&file);
            let value: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error: invalid JSON in '{}': {}", file, e);
                    std::process::exit(1);
                }
            };
            let minified = serde_json::to_string(&value).unwrap();
            if in_place {
                fs::write(&file, &minified).unwrap_or_else(|e| {
                    eprintln!("Error: cannot write '{}': {}", file, e);
                    std::process::exit(1);
                });
                println!("Minified '{}' in-place ({} bytes -> {} bytes)", file, content.len(), minified.len());
            } else {
                println!("{}", minified);
            }
        }
        Commands::Query { file, path } => {
            let content = read_file(&file);
            let value: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error: invalid JSON in '{}': {}", file, e);
                    std::process::exit(1);
                }
            };

            let keys: Vec<&str> = path.split('.').collect();
            let mut current = &value;
            for key in &keys {
                match current {
                    serde_json::Value::Object(map) => {
                        match map.get(*key) {
                            Some(v) => current = v,
                            None => {
                                eprintln!("Error: key '{}' not found in path '{}'", key, path);
                                std::process::exit(1);
                            }
                        }
                    }
                    serde_json::Value::Array(arr) => {
                        let idx: usize = match key.parse() {
                            Ok(i) => i,
                            Err(_) => {
                                eprintln!("Error: expected array index, got '{}'", key);
                                std::process::exit(1);
                            }
                        };
                        match arr.get(idx) {
                            Some(v) => current = v,
                            None => {
                                eprintln!("Error: index {} out of bounds (length {})", idx, arr.len());
                                std::process::exit(1);
                            }
                        }
                    }
                    _ => {
                        eprintln!("Error: cannot index into a non-container value");
                        std::process::exit(1);
                    }
                }
            }
            println!("{}", serde_json::to_string_pretty(current).unwrap());
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error: cannot read '{}': {}", path, e);
        std::process::exit(1);
    })
}
