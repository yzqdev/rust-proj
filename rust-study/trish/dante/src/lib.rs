//! dante - A versatile CLI tool (greet, calc, echo, now).

use std::fmt;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

/// dante - A versatile CLI tool with multiple commands
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Greet someone with a friendly message
    Greet {
        /// Name of the person to greet
        name: String,
        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u8,
    },
    /// Perform basic calculations
    Calc {
        /// First number
        a: f64,
        /// Operation: add, sub, mul, div
        #[arg(short, long)]
        op: String,
        /// Second number
        b: f64,
    },
    /// Echo the input
    Echo {
        /// Text to echo
        text: Vec<String>,
        /// Whether to echo in uppercase
        #[arg(short, long)]
        upper: bool,
    },
    /// Show current date/time
    Now {
        /// Format: date, time, datetime, timestamp
        #[arg(short, long, default_value_t = String::from("datetime"))]
        format: String,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(short, long, value_enum)]
        shell: Shell,
    },
}

/// Errors reported by the CLI layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A division by zero was requested.
    DivideByZero,
    /// The operation is not one of add/sub/mul/div.
    UnknownOperation(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DivideByZero => write!(f, "division by zero"),
            Error::UnknownOperation(op) => {
                write!(f, "unknown operation: {op}. Use add/sub/mul/div")
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// Basic arithmetic used by the `calc` command.
pub fn calc(a: f64, op: &str, b: f64) -> Result<f64> {
    match op {
        "add" | "+" => Ok(a + b),
        "sub" | "-" => Ok(a - b),
        "mul" | "*" => Ok(a * b),
        "div" | "/" => {
            if b == 0.0 {
                Err(Error::DivideByZero)
            } else {
                Ok(a / b)
            }
        }
        other => Err(Error::UnknownOperation(other.to_string())),
    }
}

/// Execute a parsed command and return the text to print on stdout.
pub fn run(cli: &Cli) -> Result<String> {
    match &cli.command {
        Commands::Greet { name, count } => Ok((0..*count)
            .map(|_| format!("Hello, {name}! Welcome to dante."))
            .collect::<Vec<_>>()
            .join("\n")),
        Commands::Calc { a, op, b } => {
            let result = calc(*a, op, *b)?;
            Ok(format!("{a} {op} {b} = {result}"))
        }
        Commands::Echo { text, upper } => {
            let msg = text.join(" ");
            Ok(if *upper { msg.to_uppercase() } else { msg })
        }
        Commands::Now { format } => {
            let now = chrono::Local::now();
            Ok(match format.as_str() {
                "date" => now.format("%Y-%m-%d").to_string(),
                "time" => now.format("%H:%M:%S").to_string(),
                "timestamp" => now.timestamp().to_string(),
                _ => now.format("%Y-%m-%d %H:%M:%S").to_string(),
            })
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

    #[test]
    fn calc_supports_all_operations() {
        assert_eq!(calc(2.0, "add", 3.0).unwrap(), 5.0);
        assert_eq!(calc(2.0, "+", 3.0).unwrap(), 5.0);
        assert_eq!(calc(5.0, "sub", 3.0).unwrap(), 2.0);
        assert_eq!(calc(4.0, "mul", 2.0).unwrap(), 8.0);
        assert_eq!(calc(9.0, "div", 3.0).unwrap(), 3.0);
    }

    #[test]
    fn calc_rejects_division_by_zero() {
        assert_eq!(calc(1.0, "div", 0.0), Err(Error::DivideByZero));
    }

    #[test]
    fn calc_rejects_unknown_operation() {
        assert_eq!(
            calc(1.0, "pow", 2.0),
            Err(Error::UnknownOperation("pow".into()))
        );
    }

    #[test]
    fn greet_repeats_by_count() {
        let cli = Cli {
            command: Commands::Greet {
                name: "Rust".into(),
                count: 2,
            },
        };
        let out = run(&cli).unwrap();
        assert_eq!(out.lines().count(), 2);
    }

    #[test]
    fn echo_uppercase() {
        let cli = Cli {
            command: Commands::Echo {
                text: vec!["hello".into(), "world".into()],
                upper: true,
            },
        };
        assert_eq!(run(&cli).unwrap(), "HELLO WORLD");
    }

    #[test]
    fn now_formats() {
        for format in ["date", "time", "datetime", "timestamp"] {
            let cli = Cli {
                command: Commands::Now {
                    format: format.into(),
                },
            };
            assert!(!run(&cli).unwrap().is_empty(), "format {format}");
        }
    }
}
